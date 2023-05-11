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

// Mark the EventListener as thread-safe
unsafe impl Send for EventListener {}
unsafe impl Sync for EventListener {}

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
            events: init_events!(Events),
        }
    }
    fn event_executor(&self, event: &Event) {
        use Event::*;
        match event {
            WorkspaceChanged(id) => arm!(id, workspace_changed_events, self),
            WorkspaceAdded(id) => arm!(id, workspace_added_events, self),
            WorkspaceDeleted(id) => arm!(id, workspace_destroyed_events, self),
            WorkspaceMoved(evend) => arm!(evend, workspace_moved_events, self),
            ActiveMonitorChanged(evend) => arm!(evend, active_monitor_changed_events, self),
            ActiveWindowChangedMerged(opt) => arm!(opt, active_window_changed_events, self),
            ActiveWindowChangedV1(_) => (),
            ActiveWindowChangedV2(_) => (),
            FullscreenStateChanged(bool) => arm!(bool, fullscreen_state_changed_events, self),
            MonitorAdded(monitor) => arm!(monitor, monitor_added_events, self),
            MonitorRemoved(monitor) => arm!(monitor, monitor_removed_events, self),
            WindowClosed(addr) => arm!(addr, window_close_events, self),
            WindowMoved(even) => arm!(even, window_moved_events, self),
            WindowOpened(even) => arm!(even, window_open_events, self),
            LayoutChanged(even) => arm!(even, keyboard_layout_change_events, self),
            SubMapChanged(map) => arm!(map, sub_map_changed_events, self),
            LayerOpened(namespace) => arm!(namespace, layer_open_events, self),
            LayerClosed(namespace) => arm!(namespace, layer_closed_events, self),
            FloatStateChanged(even) => arm!(even, float_state_events, self),
            UrgentStateChanged(even) => arm!(even, urgent_state_events, self),
            Minimize(data) => arm!(data, minimize_events, self),
            Event::Screencopy(data) => arm!(data, screencopy_events, self),
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
    pub async fn start_listener_async(&self) -> crate::Result<()> {
        use crate::unix_async::*;

        let socket_path = get_socket_path(SocketType::Listener);
        let mut stream = UnixStream::connect(socket_path).await?;

        let mut active_window_buf: Option<Option<(String, String)>> = None;
        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf).await?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];
            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed.iter() {
                if let Event::ActiveWindowChangedV1(event) = event {
                    active_window_buf = Some(event.clone());
                } else if let Event::ActiveWindowChangedV2(Some(addr)) = event {
                    match active_window_buf.clone() {
                        Some(Some((class, title))) => {
                            self.event_executor(&Event::ActiveWindowChangedMerged(Some(
                                WindowEventData {
                                    window_class: class.to_string(),
                                    window_title: title.to_string(),
                                    window_address: addr.clone(),
                                },
                            )));
                        }
                        Some(None) => {}
                        None => {}
                    };
                } else if let Event::ActiveWindowChangedV2(None) = event {
                    self.event_executor(&Event::ActiveWindowChangedMerged(None));
                } else {
                    self.event_executor(event);
                }
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
    pub fn start_listener(self) -> crate::Result<()> {
        use io::prelude::*;
        use std::os::unix::net::UnixStream;

        let socket_path = get_socket_path(SocketType::Listener);
        let mut stream = UnixStream::connect(socket_path)?;

        let mut active_window_buf: Option<Option<(String, String)>> = None;
        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];
            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed.iter() {
                if let Event::ActiveWindowChangedV1(event) = event {
                    active_window_buf = Some(event.clone());
                } else if let Event::ActiveWindowChangedV2(Some(addr)) = event {
                    match active_window_buf.clone() {
                        Some(Some((class, title))) => {
                            self.event_executor(&Event::ActiveWindowChangedMerged(Some(
                                WindowEventData {
                                    window_class: class.to_string(),
                                    window_title: title.to_string(),
                                    window_address: addr.clone(),
                                },
                            )));
                        }
                        Some(None) => {}
                        None => {}
                    };
                } else if let Event::ActiveWindowChangedV2(None) = event {
                    self.event_executor(&Event::ActiveWindowChangedMerged(None));
                } else {
                    self.event_executor(event);
                }
            }
        }

        Ok(())
    }
}
