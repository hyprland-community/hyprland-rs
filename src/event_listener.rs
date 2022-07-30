use crate::shared::{get_socket_path, SocketType, WorkspaceId};
use regex::{Regex, RegexSet};
use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;

#[derive(Debug)]
pub struct WindowEventData(String, String);

#[derive(Debug)]
pub struct MonitorEventData(String, WorkspaceId);

#[derive(Debug)]
pub enum Event {
    WorkspaceChanged(WorkspaceId),
    WorkspaceDeleted(WorkspaceId),
    WorkspaceAdded(WorkspaceId),
    ActiveWindowChanged(Option<WindowEventData>),
    ActiveMonitorChanged(MonitorEventData),
    FullscreenStateChanged(bool),
    MonitorAdded(String),
    MonitorRemoved(String),
}

fn event_parser(event: String) -> io::Result<Vec<Event>> {
    lazy_static! {
        static ref EVENT_SET: RegexSet = RegexSet::new(&[
            r"\bworkspace>>(?P<workspace>[0-9]{1,2}|)",
            r"destroyworkspace>>(?P<workspace>[0-9]{1,2})",
            r"createworkspace>>(?P<workspace>[0-9]{1,2})",
            r"activemon>>(?P<monitor>.*),(?P<workspace>[0-9]{1,2})",
            r"activewindow>>(?P<class>.*),(?P<title>.*)",
            r"fullscreen>>(?P<state>0|1)",
            r"monitorremoved>>(?P<monitor>.*)",
            r"monitoradded>>(?P<monitor>.*)"
        ])
        .unwrap();
        static ref EVENT_REGEXES: Vec<Regex> = EVENT_SET
            .patterns()
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();
    }

    let event_iter = event.trim().split('\n');

    let mut events: Vec<Event> = vec![];

    for item in event_iter {
        let matches = EVENT_SET.matches(item);
        let matches_event: Vec<_> = matches.into_iter().collect();
        let captures = if !EVENT_REGEXES.is_empty() && !matches_event.is_empty() {
            EVENT_REGEXES[matches_event[0]].captures(item).unwrap()
        } else {
            panic!("something has went down -{:#?}-", matches_event)
        };

        if matches_event.len() == 1 {
            match matches_event[0] {
                0 => {
                    // WorkspaceChanged
                    let captured = &captures["workspace"];
                    let workspace = if !captured.is_empty() {
                        captured.parse::<u8>().expect("Not a valid int")
                    } else {
                        1_u8
                    };
                    events.push(Event::WorkspaceChanged(workspace));
                }
                1 => {
                    // destroyworkspace
                    let workspace = captures["workspace"].parse::<u8>().unwrap();
                    events.push(Event::WorkspaceDeleted(workspace));
                }
                2 => {
                    // WorkspaceAdded
                    let workspace = captures["workspace"].parse::<u8>().unwrap();
                    events.push(Event::WorkspaceAdded(workspace));
                }
                3 => {
                    // ActiveMonitorChanged
                    let monitor = &captures["monitor"];
                    let workspace = captures["workspace"].parse::<u8>().unwrap();
                    events.push(Event::ActiveMonitorChanged(MonitorEventData(
                        monitor.to_string(),
                        workspace,
                    )));
                }
                4 => {
                    // ActiveWindowChanged
                    let class = &captures["class"];
                    let title = &captures["title"];
                    if !class.is_empty() && !title.is_empty() {
                        events.push(Event::ActiveWindowChanged(Some(WindowEventData(
                            class.to_string(),
                            title.to_string(),
                        ))));
                    } else {
                        events.push(Event::ActiveWindowChanged(None));
                    }
                }
                5 => {
                    // FullscreenStateChanged
                    let state = &captures["state"] == "0";
                    events.push(Event::FullscreenStateChanged(state))
                }
                6 => {
                    // MonitorRemoved
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorRemoved(monitor.to_string()));
                }
                7 => {
                    // MonitorAdded
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorAdded(monitor.to_string()));
                }
                _ => panic!("How did this happen?"),
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown event"));
        }
    }

    Ok(events)
}

pub struct EventListener<'a> {
    workspace_changed_events: Vec<&'a dyn Fn(WorkspaceId)>,
    workspace_added_events: Vec<&'a dyn Fn(WorkspaceId)>,
    workspace_destroyed_events: Vec<&'a dyn Fn(WorkspaceId)>,
    active_monitor_changed_events: Vec<&'a dyn Fn(MonitorEventData)>,
    active_window_changed_events: Vec<&'a dyn Fn(Option<WindowEventData>)>,
    fullscreen_state_changed_events: Vec<&'a dyn Fn(bool)>,
    monitor_removed_events: Vec<&'a dyn Fn(String)>,
    monitor_added_events: Vec<&'a dyn Fn(String)>,
}

impl EventListener<'_> {
    pub fn new() -> EventListener<'static> {
        EventListener {
            workspace_changed_events: vec![],
            workspace_added_events: vec![],
            workspace_destroyed_events: vec![],
            active_monitor_changed_events: vec![],
            active_window_changed_events: vec![],
            fullscreen_state_changed_events: vec![],
            monitor_removed_events: vec![],
            monitor_added_events: vec![],
        }
    }

    pub fn add_workspace_change_handler(&mut self, f: &'static dyn Fn(WorkspaceId)) {
        self.workspace_changed_events.push(f);
    }

    pub fn add_workspace_added_handler(&mut self, f: &'static dyn Fn(WorkspaceId)) {
        self.workspace_added_events.push(f);
    }
    pub fn add_workspace_destroy_handler(&mut self, f: &'static dyn Fn(WorkspaceId)) {
        self.workspace_destroyed_events.push(f);
    }
    pub fn add_active_monitor_change_handler(&mut self, f: &'static dyn Fn(MonitorEventData)) {
        self.active_monitor_changed_events.push(f);
    }
    pub fn add_active_window_change_handler(
        &mut self,
        f: &'static dyn Fn(Option<WindowEventData>),
    ) {
        self.active_window_changed_events.push(f);
    }
    pub fn add_fullscreen_state_change_handler(&mut self, f: &'static dyn Fn(bool)) {
        self.fullscreen_state_changed_events.push(f);
    }
    pub fn add_monitor_added_handler(&mut self, f: &'static dyn Fn(String)) {
        self.monitor_added_events.push(f);
    }
    pub fn add_monitor_removed_handler(&mut self, f: &'static dyn Fn(String)) {
        self.monitor_removed_events.push(f);
    }

    pub async fn start_listener(&self) -> io::Result<()> {
        let socket_path = get_socket_path(SocketType::Listener);

        let mut stream = UnixStream::connect(socket_path).await?;

        let mut buf = [0; 4096];

        loop {
            stream.readable().await?;
            let num_read = stream.read(&mut buf).await?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];

            let string = match String::from_utf8(buf.to_vec()) {
                Ok(str) => str,
                Err(error) => panic!("a error has occured {error:#?}"),
            };

            let parsed: Vec<Event> = match event_parser(string) {
                Ok(vec) => vec,
                Err(error) => panic!("a error has occured {error:#?}"),
            };

            for event in parsed.iter() {
                match event {
                    Event::WorkspaceChanged(id) => {
                        let events = &self.workspace_changed_events;
                        for item in events.iter() {
                            item(*id)
                        }
                    }
                    Event::WorkspaceAdded(id) => {
                        let events = &self.workspace_added_events;
                        for item in events.iter() {
                            item(*id)
                        }
                    }
                    Event::WorkspaceDeleted(id) => {
                        let events = &self.workspace_destroyed_events;
                        for item in events.iter() {
                            item(*id)
                        }
                    }
                    Event::ActiveMonitorChanged(MonitorEventData(monitor, id)) => {
                        let events = &self.active_monitor_changed_events;
                        for item in events.iter() {
                            item(MonitorEventData(monitor.clone(), *id))
                        }
                    }
                    Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => {
                        let events = &self.active_window_changed_events;
                        for item in events.iter() {
                            item(Some(WindowEventData(class.clone(), title.clone())))
                        }
                    }
                    Event::ActiveWindowChanged(None) => {
                        let events = &self.active_window_changed_events;
                        for item in events.iter() {
                            item(None)
                        }
                    }
                    Event::FullscreenStateChanged(bool) => {
                        let events = &self.fullscreen_state_changed_events;
                        for item in events.iter() {
                            item(*bool)
                        }
                    }
                    Event::MonitorAdded(monitor) => {
                        let events = &self.monitor_added_events;
                        for item in events.iter() {
                            item(monitor.clone())
                        }
                    }
                    Event::MonitorRemoved(monitor) => {
                        let events = &self.monitor_removed_events;
                        for item in events.iter() {
                            item(monitor.clone())
                        }
                    }
                }
            }
        }

        Ok(())
    }
    pub fn start_listener_blocking(self) -> io::Result<()> {
        use tokio::runtime::Runtime;

        let rt = Runtime::new()?;

        rt.block_on(self.start_listener())
    }
}
