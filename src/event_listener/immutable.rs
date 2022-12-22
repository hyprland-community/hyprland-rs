use crate::shared::*;
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
/// listener.start_listener(); // or `.start_listener_async().await` if async
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
                workspace_moved_events: vec![],
                active_monitor_changed_events: vec![],
                active_window_changed_events: vec![],
                fullscreen_state_changed_events: vec![],
                monitor_removed_events: vec![],
                monitor_added_events: vec![],
                window_open_events: vec![],
                window_close_events: vec![],
                window_moved_events: vec![],
                keyboard_layout_change_events: vec![],
                sub_map_changed_events: vec![],
                layer_open_events: vec![],
                layer_closed_events: vec![],
            },
        }
    }

    add_listener!(
        reg add_workspace_change_handler,
        workspace_changed_events,
        WorkspaceType,
        "This method adds a event to the listener which executes on workspace change",
        r#"listener.add_workspace_change_handler(|id| println!("changed workspace to {id:?}"));"#
    );

    add_listener!(
        reg add_workspace_added_handler,
        workspace_added_events,
        WorkspaceType,
        "This method add a event to the listener which executes when a new workspace is created",
        r#"listener.add_workspace_added_handler(|id| println!("workspace {id:?} was added"));"#
    );

    add_listener!(
        reg add_workspace_destroy_handler,
        workspace_destroyed_events,
        WorkspaceType,
        "This method to add a event to the listener which executes when a workspace is destroyed",
        r#"listener.add_workspace_destroy_handler(|id| println!("workspace {id:?} was destroyed"));"#
    );

    add_listener!(
        reg add_workspace_moved_handler,
        workspace_moved_events,
        MonitorEventData,
        "This method to add a event to the listener which executes when a workspace is moved",
        r#"listener.add_workspace_moved_handler(|id| println!("workspace {id:?} was moved"));"#
    );

    add_listener!(
        reg add_active_monitor_change_handler,
        active_monitor_changed_events,
        MonitorEventData,
        "This method add a event to the listener which executes when the active monitor is changed",
        r#"listener.add_active_monitor_change_handler(|data| println!("Active monitor changed to {data:#?}"));"#
    );

    add_listener!(
        reg add_active_window_change_handler,
        active_window_changed_events,
        Option<WindowEventData>,
        "This method add a event to the listener which executes when the active window is changed",
        r#"listener.add_active_window_change_handler(|data| println!("Active window changed: {data:#?}"));"#
    );

    add_listener!(
        reg add_fullscreen_state_change_handler,
        fullscreen_state_changed_events,
        bool,
        "This method adds an event to the listener which executes when the active monitor is changed",
        r#"listener.add_fullscreen_state_change_handler(|state| println!("Fullscreen is on: {state}"));"#
    );

    add_listener!(
        reg add_monitor_added_handler,
        monitor_added_events,
        String,
        "This method adds an event to the listener which executes when a new monitor is added",
        r#"listener.add_monitor_added_handler(|data| println!("Monitor added: {data}"));"#
    );

    add_listener!(
        reg add_monitor_removed_handler,
        monitor_removed_events,
        String,
        "This method adds an event to the listener which executes when a monitor is removed",
        r#"listener.add_monitor_removed_handler(|data| println!("Monitor removed: {data}"));"#
    );

    add_listener!(
        reg add_window_open_handler,
        window_open_events,
        WindowOpenEvent,
        "This method adds an event to the listener which executes when a window is opened",
        r#"listener.add_window_open_handler(|data| println!("Window opened: {data:#?}"));"#
    );

    add_listener!(
        reg add_window_close_handler,
        window_close_events,
        Address,
        "This method adds an event to the listener which executes when a window is closed",
        r#"listener.add_window_close_handler(|data| println!("Window closed: {data}"));"#
    );

    add_listener!(
        reg add_window_moved_handler,
        window_moved_events,
        WindowMoveEvent,
        "This method adds an event to the listener which executes when a window is moved",
        r#"listener.add_window_moved_handler(|data| println!("Window moved: {data:#?}"));"#
    );

    add_listener!(
        reg add_keyboard_layout_change_handler,
        keyboard_layout_change_events,
        LayoutEvent,
        "This method adds an event to the listener which executes when the keyboard layout is changed",
        r#"listener.add_keyboard_layout_change_handler(|data| println!("Layout changed: {data:#?}"));"#
    );

    add_listener!(
        reg add_sub_map_change_handler,
        sub_map_changed_events,
        String,
        "This method adds an event to the listener which executes when the sub map is changed",
        r#"listener.add_sub_map_change_handler(|data| println!("Submap changed: {data}"));"#
    );

    add_listener!(
        reg add_layer_open_handler,
        layer_open_events,
        String,
        "This method adds an event to the listener which executes when a new layer is opened",
        r#"listener.add_layer_open_handler(|data| println!("Layer opened: {data}"));"#
    );

    add_listener!(
        reg add_layer_closed_handler,
        layer_closed_events,
        String,
        "This method adds an event to the listener which executes when a layer is closed",
        r#"listener.add_layer_closed_handler(|data| println!("Layer closed: {data}"));"#
    );

    fn event_executor(&self, event: &Event) {
        match event {
            Event::WorkspaceChanged(id) => arm_sync!(id.clone(), workspace_changed_events, self),
            Event::WorkspaceAdded(id) => arm_sync!(id.clone(), workspace_added_events, self),
            Event::WorkspaceDeleted(id) => arm_sync!(id.clone(), workspace_destroyed_events, self),
            Event::WorkspaceMoved(evend) => arm_sync!(evend.clone(), workspace_moved_events, self),
            Event::ActiveMonitorChanged(evend) => {
                arm_sync!(evend.clone(), active_monitor_changed_events, self)
            }
            Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => arm_sync!(
                Some(WindowEventData(class.clone(), title.clone())),
                active_window_changed_events,
                self
            ),
            Event::ActiveWindowChanged(None) => arm_sync!(None, active_window_changed_events, self),
            Event::FullscreenStateChanged(bool) => {
                arm_sync!(*bool, fullscreen_state_changed_events, self)
            }
            Event::MonitorAdded(monitor) => arm_sync!(monitor.clone(), monitor_added_events, self),
            Event::MonitorRemoved(monitor) => {
                arm_sync!(monitor.clone(), monitor_removed_events, self)
            }
            Event::WindowClosed(addr) => arm_sync!(addr.clone(), window_close_events, self),
            Event::WindowMoved(even) => arm_sync!(even.clone(), window_moved_events, self),
            Event::WindowOpened(even) => arm_sync!(even.clone(), window_open_events, self),
            Event::LayoutChanged(even) => {
                arm_sync!(even.clone(), keyboard_layout_change_events, self)
            }
            Event::SubMapChanged(map) => {
                let events = &self.events.sub_map_changed_events;
                for item in events.iter() {
                    execute_closure(item, map.clone())
                }
            }
            Event::LayerOpened(namespace) => arm_sync!(namespace.clone(), layer_open_events, self),
            Event::LayerClosed(namespace) => {
                arm_sync!(namespace.clone(), layer_closed_events, self)
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
    /// listener.add_workspace_change_handler(|id| println!("changed workspace to {id:?}"));
    /// listener.start_listener_async().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_listener_async(&self) -> HResult<()> {
        use tokio::io::AsyncReadExt;
        use tokio::net::UnixStream;

        let socket_path = get_socket_path(SocketType::Listener);

        let mut stream = UnixStream::connect(socket_path).await?;

        loop {
            let mut buf = [0; 2048];

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
    /// listener.add_workspace_change_handler(&|id| println!("changed workspace to {id:?}"));
    /// listener.start_listener();
    /// ```
    pub fn start_listener(self) -> HResult<()> {
        use io::prelude::*;
        use std::os::unix::net::UnixStream;

        let socket_path = get_socket_path(SocketType::Listener);

        let mut stream = UnixStream::connect(socket_path)?;

        loop {
            let mut buf = [0; 2048];

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
    }
}
