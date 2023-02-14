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

    add_listener!(workspace_change d, WorkspaceType, "on workspace change", "changed workspace to" => id);
    add_listener!(workspace_added, WorkspaceType, "a workspace is created", "workspace was added" => id);
    add_listener!(workspace_destroy ed, WorkspaceType, "a workspace is destroyed", "workspace was destroyed" => id);
    add_listener!(workspace_moved, MonitorEventData, "a workspace is moved", "workspace was moved" => id);
    add_listener!(active_monitor_change d, MonitorEventData, "the active monitor is changed", "Active monitor changed to" => data);
    add_listener!(active_window_change d, Option<WindowEventData>, "the active window is changed", "Active window changed" => data);
    add_listener!(fullscreen_state_change d, bool, "the active monitor is changed", "Fullscreen is on" => state);
    add_listener!(monitor_added, String, "a new monitor is added", "Monitor added" => data);
    add_listener!(monitor_removed, String, "a monitor is removed", "Monitor removed" => data);
    add_listener!(window_open, WindowOpenEvent, "a window is opened", "Window opened" => data);
    add_listener!(window_close, Address, "a window is closed", "Window closed" => data);
    add_listener!(window_moved, WindowMoveEvent, "a window is moved", "Window moved" => data);
    add_listener!(keyboard_layout_change, LayoutEvent, "the keyboard layout is changed", "Layout changed" => data);
    add_listener!(sub_map_change d, String, "the sub map is changed", "Submap changed" => data);
    add_listener!(layer_open, String, "a new layer is opened", "Layer opened" => data);
    add_listener!(layer_closed, String, "a layer is closed", "Layer closed" => data);
    add_listener!(float_state, WindowFloatEventData, "the float state of a window is changed", "Float state changed" => data);
    add_listener!(urgent_state, Address, "the urgent state of a window is changed", "urgent state changed" => data);

    fn event_executor(&self, event: &Event) {
        match event {
            Event::WorkspaceChanged(id) => arm!(id.clone(), workspace_changed_events, self),
            Event::WorkspaceAdded(id) => arm!(id.clone(), workspace_added_events, self),
            Event::WorkspaceDeleted(id) => arm!(id.clone(), workspace_destroyed_events, self),
            Event::WorkspaceMoved(evend) => arm!(evend.clone(), workspace_moved_events, self),
            Event::ActiveMonitorChanged(evend) => {
                arm!(evend.clone(), active_monitor_changed_events, self)
            }
            Event::ActiveWindowChangedMerged(Some(event)) => {
                arm!(Some(event.clone()), active_window_changed_events, self)
            }
            Event::ActiveWindowChangedMerged(None) => {
                arm!(None, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => {
                arm!(*bool, fullscreen_state_changed_events, self)
            }
            Event::MonitorAdded(monitor) => arm!(monitor.clone(), monitor_added_events, self),
            Event::MonitorRemoved(monitor) => {
                arm!(monitor.clone(), monitor_removed_events, self)
            }
            Event::WindowClosed(addr) => arm!(addr.clone(), window_close_events, self),
            Event::WindowMoved(even) => arm!(even.clone(), window_moved_events, self),
            Event::WindowOpened(even) => arm!(even.clone(), window_open_events, self),
            Event::LayoutChanged(even) => {
                arm!(even.clone(), keyboard_layout_change_events, self)
            }
            Event::SubMapChanged(map) => arm!(map.clone(), sub_map_changed_events, self),
            Event::LayerOpened(namespace) => arm!(namespace.clone(), layer_open_events, self),
            Event::LayerClosed(namespace) => {
                arm!(namespace.clone(), layer_closed_events, self)
            }
            Event::FloatStateChanged(even) => arm!(even.clone(), float_state_events, self),
            Event::UrgentStateChanged(even) => arm!(even.clone(), urgent_state_events, self),
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
                                WindowEventData(class.to_string(), title.to_string(), addr.clone()),
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
    pub fn start_listener(self) -> HResult<()> {
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
                                WindowEventData(class.to_string(), title.to_string(), addr.clone()),
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
