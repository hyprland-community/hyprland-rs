use crate::shared::{get_socket_path, SocketType, WorkspaceId};
use std::io;

use crate::event_listener::shared::*;

/// This struct is used for adding event handlers and executing them on events
/// # The Event Listener
///
/// This struct holds what you need to create a event listener
///
/// ## Usage
///
/// ```rust, no_run
/// use hyprland::event_listener::EventListener;
/// let mut listener = EventListener::new(); // creates a new listener
/// // add a event handler which will be ran when this event happens
/// listener.add_workspace_change_handler(|data| println!("{:#?}", data));
/// listener.start_listener_blocking(); // or `.start_listener().await` if async
/// ```
pub struct EventListener {
    pub(crate) events: Events,
}

impl Default for EventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl EventListener {
    /// This method creates a new EventListener instance
    ///
    /// ```rust
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> EventListener {
        EventListener {
            events: Events {
                workspace_changed_events: vec![],
                workspace_added_events: vec![],
                workspace_destroyed_events: vec![],
                active_monitor_changed_events: vec![],
                active_window_changed_events: vec![],
                fullscreen_state_changed_events: vec![],
                monitor_removed_events: vec![],
                monitor_added_events: vec![],
            },
        }
    }

    /// This method adds a event to the listener which executes on workspace change
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id| println!("changed workspace to {id}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_change_handler(&mut self, f: impl Fn(WorkspaceId) + 'static) {
        self.events
            .workspace_changed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new workspace is created
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_added_handler(|id| println!("workspace {id} was added"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_added_handler(&mut self, f: impl Fn(WorkspaceId) + 'static) {
        self.events
            .workspace_added_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new workspace is created
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_destroy_handler(|id| println!("workspace {id} was destroyed"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_destroy_handler(&mut self, f: impl Fn(WorkspaceId) + 'static) {
        self.events
            .workspace_destroyed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active monitor is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_active_monitor_change_handler(|data| println!("Active Monitor changed: {data:#?}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_active_monitor_change_handler(&mut self, f: impl Fn(MonitorEventData) + 'static) {
        self.events
            .active_monitor_changed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active window is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_active_window_change_handler(|data| println!("Active window changed: {data:#?}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_active_window_change_handler(
        &mut self,
        f: impl Fn(Option<WindowEventData>) + 'static,
    ) {
        self.events
            .active_window_changed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active monitor is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_fullscreen_state_change_handler(|state| println!("Fullscreen is on: {state}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_fullscreen_state_change_handler(&mut self, f: impl Fn(bool) + 'static) {
        self.events
            .fullscreen_state_changed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new monitor is added
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_monitor_added_handler(|data| println!("Monitor added: {data}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_monitor_added_handler(&mut self, f: impl Fn(String) + 'static) {
        self.events
            .monitor_added_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a monitor is removed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_monitor_removed_handler(|data| println!("Monitor removed: {data}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_monitor_removed_handler(&mut self, f: impl Fn(String) + 'static) {
        self.events
            .monitor_removed_events
            .push(EventTypes::Regular(Box::new(f)));
    }

    fn event_executor(&self, event: &Event) {
        match event {
            Event::WorkspaceChanged(id) => {
                let events = &self.events.workspace_changed_events;
                for item in events.iter() {
                    execute_closure(item, *id);
                }
            }
            Event::WorkspaceAdded(id) => {
                let events = &self.events.workspace_added_events;
                for item in events.iter() {
                    execute_closure(item, *id);
                }
            }
            Event::WorkspaceDeleted(id) => {
                let events = &self.events.workspace_destroyed_events;
                for item in events.iter() {
                    execute_closure(item, *id);
                }
            }
            Event::ActiveMonitorChanged(MonitorEventData(monitor, id)) => {
                let events = &self.events.active_monitor_changed_events;
                for item in events.iter() {
                    execute_closure(item, MonitorEventData(monitor.clone(), *id));
                }
            }
            Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    execute_closure(item, Some(WindowEventData(class.clone(), title.clone())));
                }
            }
            Event::ActiveWindowChanged(None) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    execute_closure(item, None);
                }
            }
            Event::FullscreenStateChanged(bool) => {
                let events = &self.events.fullscreen_state_changed_events;
                for item in events.iter() {
                    execute_closure(item, *bool);
                }
            }
            Event::MonitorAdded(monitor) => {
                let events = &self.events.monitor_added_events;
                for item in events.iter() {
                    execute_closure(item, monitor.clone());
                }
            }
            Event::MonitorRemoved(monitor) => {
                let events = &self.events.monitor_removed_events;
                for item in events.iter() {
                    execute_closure(item, monitor.clone());
                }
            }
        }
    }

    /// This method starts the event listener (async)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// # async fn function() -> std::io::Result<()> {
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id| println!("changed workspace to {id}"));
    /// listener.start_listener().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_listener(&self) -> io::Result<()> {
        use tokio::io::AsyncReadExt;
        use tokio::net::UnixStream;

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
                self.event_executor(event);
            }
        }

        Ok(())
    }

    /// This method starts the event listener (blocking)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(&|id| println!("changed workspace to {id}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn start_listener_blocking(self) -> io::Result<()> {
        use io::prelude::*;
        use std::os::unix::net::UnixStream;

        let socket_path = get_socket_path(SocketType::Listener);

        let mut stream = UnixStream::connect(socket_path)?;

        let mut buf = [0; 4096];

        loop {
            //stream.readable()?;
            let num_read = stream.read(&mut buf)?;
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
                self.event_executor(event);
            }
        }

        Ok(())
        // use tokio::runtime::Runtime;
        //
        // lazy_static! {
        //     static ref RT: Runtime = match Runtime::new() {
        //         Ok(run) => run,
        //         Err(e) => panic!("Error creating tokio runtime: {e}"),
        //     };
        // }
        //
        // RT.block_on(self.start_listener())
    }
}
