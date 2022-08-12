use crate::shared::{get_socket_path, SocketType, WorkspaceId};
use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;

use crate::event_listener::shared::*;

/// This struct is used for adding event handlers and executing them on events
/// # The Event Listener
///
/// This struct holds what you need to create a event listener
///
/// ## Usage
///
/// ```rust, no_run
/// use hyprland::event_listener::EventListenerMutable as EventListener;
/// let mut listener = EventListener::new(); // creates a new listener
/// // add a event handler which will be ran when this event happens
/// listener.add_workspace_change_handler(|data, _| println!("{:#?}", data));
/// listener.start_listener_blocking(); // or `.start_listener().await` if async
/// ```
pub struct EventListener {
    pub(crate) events: Events,
    /// The state of some of the events
    pub state: State,
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
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> EventListener {
        use crate::data::blocking::{get_active_workspace, get_fullscreen_state, get_monitors};
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
            state: State {
                active_workspace: match get_active_workspace() {
                    Ok(work) => work.id,
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
                active_monitor: match get_monitors() {
                    Ok(monitors) => match monitors.iter().find(|item| item.focused) {
                        Some(mon) => mon.name.clone(),
                        None => panic!("No active monitor?"),
                    },
                    Err(e) => panic!("A error occured when parsing json with serde {e}"),
                },
                fullscreen_state: match get_fullscreen_state() {
                    Ok(fstate) => fstate,
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
            },
        }
    }

    /// This method adds a event to the listener which executes on workspace change
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_change_handler(&mut self, f: impl Fn(WorkspaceId, &mut State) + 'static) {
        self.events
            .workspace_changed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new workspace is created
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_added_handler(|id, _| println!("workspace {id} was added"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_added_handler(&mut self, f: impl Fn(WorkspaceId, &mut State) + 'static) {
        self.events
            .workspace_added_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new workspace is created
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_destroy_handler(|id, _| println!("workspace {id} was destroyed"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_workspace_destroy_handler(&mut self, f: impl Fn(WorkspaceId, &mut State) + 'static) {
        self.events
            .workspace_destroyed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active monitor is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_active_monitor_change_handler(|data, _| println!("Active Monitor changed: {data:#?}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_active_monitor_change_handler(
        &mut self,
        f: impl Fn(MonitorEventData, &mut State) + 'static,
    ) {
        self.events
            .active_monitor_changed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active window is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_active_window_change_handler(|data, _| println!("Active window changed: {data:#?}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_active_window_change_handler(
        &mut self,
        f: impl Fn(Option<WindowEventData>, &mut State) + 'static,
    ) {
        self.events
            .active_window_changed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when the active monitor is changed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_fullscreen_state_change_handler(|state, _| println!("Fullscreen is on: {state}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_fullscreen_state_change_handler(&mut self, f: impl Fn(bool, &mut State) + 'static) {
        self.events
            .fullscreen_state_changed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a new monitor is added
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_monitor_added_handler(|data, _| println!("Monitor added: {data}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_monitor_added_handler(&mut self, f: impl Fn(String, &mut State) + 'static) {
        self.events
            .monitor_added_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    /// This method add a event to the listener which executes when a monitor is removed
    ///
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_monitor_removed_handler(|data, _| println!("Monitor removed: {data}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn add_monitor_removed_handler(&mut self, f: impl Fn(String, &mut State) + 'static) {
        self.events
            .monitor_removed_events
            .push(EventTypes::MutableState(Box::new(f)));
    }

    async fn event_executor(&mut self, event: &Event) -> io::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => {
                let handlers = &self.events.workspace_changed_events;
                self.state.active_workspace = *id;
                for item in handlers.iter() {
                    let new_state = execute_closure_mut(self.state.clone(), item, *id).await?;
                    self.state = new_state;
                }
            }
            Event::WorkspaceAdded(id) => {
                let events = &self.events.workspace_added_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut(self.state.clone(), item, *id).await?;
                    self.state = new_state;
                }
            }
            Event::WorkspaceDeleted(id) => {
                let events = &self.events.workspace_destroyed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut(self.state.clone(), item, *id).await?;
                    self.state = new_state;
                }
            }
            Event::ActiveMonitorChanged(MonitorEventData(monitor, id)) => {
                let events = &self.events.active_monitor_changed_events;
                self.state.active_monitor = monitor.clone();
                for item in events.iter() {
                    let new_state = execute_closure_mut(
                        self.state.clone(),
                        item,
                        MonitorEventData(monitor.clone(), *id),
                    )
                    .await?;
                    self.state = new_state;
                }
            }
            Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut(
                        self.state.clone(),
                        item,
                        Some(WindowEventData(class.clone(), title.clone())),
                    )
                    .await?;
                    self.state = new_state;
                }
            }
            Event::ActiveWindowChanged(None) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut(self.state.clone(), item, None).await?;
                    self.state = new_state;
                }
            }
            Event::FullscreenStateChanged(bool) => {
                let events = &self.events.fullscreen_state_changed_events;
                self.state.fullscreen_state = *bool;
                for item in events.iter() {
                    let new_state = execute_closure_mut(self.state.clone(), item, *bool).await?;
                    self.state = new_state;
                }
            }
            Event::MonitorAdded(monitor) => {
                let events = &self.events.monitor_added_events;
                for item in events.iter() {
                    let new_state =
                        execute_closure_mut(self.state.clone(), item, monitor.clone()).await?;
                    self.state = new_state;
                }
            }
            Event::MonitorRemoved(monitor) => {
                let events = &self.events.monitor_removed_events;
                for item in events.iter() {
                    let new_state =
                        execute_closure_mut(self.state.clone(), item, monitor.clone()).await?;
                    self.state = new_state;
                }
            }
        }
        Ok(())
    }

    fn event_executor_sync(&mut self, event: &Event) -> io::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => {
                let handlers = &self.events.workspace_changed_events;
                self.state.active_workspace = *id;
                for item in handlers.iter() {
                    let new_state = execute_closure_mut_sync(self.state.clone(), item, *id)?;
                    self.state = new_state;
                }
            }
            Event::WorkspaceAdded(id) => {
                let events = &self.events.workspace_added_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(self.state.clone(), item, *id)?;
                    self.state = new_state;
                }
            }
            Event::WorkspaceDeleted(id) => {
                let events = &self.events.workspace_destroyed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(self.state.clone(), item, *id)?;
                    self.state = new_state;
                }
            }
            Event::ActiveMonitorChanged(MonitorEventData(monitor, id)) => {
                let events = &self.events.active_monitor_changed_events;
                self.state.active_monitor = monitor.clone();
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(
                        self.state.clone(),
                        item,
                        MonitorEventData(monitor.clone(), *id),
                    )?;
                    self.state = new_state;
                }
            }
            Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(
                        self.state.clone(),
                        item,
                        Some(WindowEventData(class.clone(), title.clone())),
                    )?;
                    self.state = new_state;
                }
            }
            Event::ActiveWindowChanged(None) => {
                let events = &self.events.active_window_changed_events;
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(self.state.clone(), item, None)?;
                    self.state = new_state;
                }
            }
            Event::FullscreenStateChanged(bool) => {
                let events = &self.events.fullscreen_state_changed_events;
                self.state.fullscreen_state = *bool;
                for item in events.iter() {
                    let new_state = execute_closure_mut_sync(self.state.clone(), item, *bool)?;
                    self.state = new_state;
                }
            }
            Event::MonitorAdded(monitor) => {
                let events = &self.events.monitor_added_events;
                for item in events.iter() {
                    let new_state =
                        execute_closure_mut_sync(self.state.clone(), item, monitor.clone())?;
                    self.state = new_state;
                }
            }
            Event::MonitorRemoved(monitor) => {
                let events = &self.events.monitor_removed_events;
                for item in events.iter() {
                    let new_state =
                        execute_closure_mut_sync(self.state.clone(), item, monitor.clone())?;
                    self.state = new_state;
                }
            }
        }
        Ok(())
    }

    /// This method starts the event listener (async)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// # async fn function() -> std::io::Result<()> {
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id}"));
    /// listener.start_listener().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_listener(&mut self) -> io::Result<()> {
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
                self.event_executor(event).await?;
            }
        }

        Ok(())
    }

    /// This method starts the event listener (blocking)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id}"));
    /// listener.start_listener_blocking();
    /// ```
    pub fn start_listener_blocking(mut self) -> io::Result<()> {
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
                self.event_executor_sync(event)?;
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
