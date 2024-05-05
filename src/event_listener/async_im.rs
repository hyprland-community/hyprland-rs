use crate::shared::*;

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
pub struct AsyncEventListener {
    pub(crate) events: AsyncEvents,
}

// Mark the EventListener as thread-safe
impl Default for AsyncEventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl HasAsyncExecutor for AsyncEventListener {
    async fn event_executor_async(&mut self, event: Event) -> crate::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => arm_async!(id, workspace_changed_events, self),
            Event::WorkspaceAdded(id) => arm_async!(id, workspace_added_events, self),
            Event::WorkspaceDeleted(data) => {
                arm_async!(data, workspace_destroyed_events, self)
            }
            Event::WorkspaceMoved(evend) => arm_async!(evend, workspace_moved_events, self),
            Event::WorkspaceRename(even) => arm_async!(even, workspace_rename_events, self),
            Event::ActiveMonitorChanged(evend) => {
                arm_async!(evend, active_monitor_changed_events, self)
            }
            Event::ActiveWindowChangedMerged(event) => {
                arm_async!(event, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => {
                arm_async!(bool, fullscreen_state_changed_events, self)
            }
            Event::MonitorAdded(monitor) => arm_async!(monitor, monitor_added_events, self),
            Event::MonitorRemoved(monitor) => arm_async!(monitor, monitor_removed_events, self),
            Event::WindowClosed(addr) => arm_async!(addr, window_close_events, self),
            Event::WindowMoved(even) => arm_async!(even, window_moved_events, self),
            Event::WindowOpened(even) => arm_async!(even, window_open_events, self),
            Event::LayoutChanged(even) => arm_async!(even, keyboard_layout_change_events, self),
            Event::SubMapChanged(map) => arm_async!(map, sub_map_changed_events, self),
            Event::LayerOpened(namespace) => arm_async!(namespace, layer_open_events, self),
            Event::LayerClosed(namespace) => arm_async!(namespace, layer_closed_events, self),
            Event::FloatStateChanged(even) => arm_async!(even, float_state_events, self),
            Event::UrgentStateChanged(even) => arm_async!(even, urgent_state_events, self),
            Event::Minimize(data) => arm_async!(data, minimize_events, self),
            Event::WindowTitleChanged(addr) => arm_async!(addr, window_title_changed_events, self),
            Event::Screencast(data) => arm_async!(data, screencast_events, self),
        }
        Ok(())
    }
}

impl AsyncEventListener {
    /// This method creates a new EventListener instance
    ///
    /// ```rust
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> Self {
        Self {
            events: init_events!(AsyncEvents),
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
    pub async fn start_listener_async(&mut self) -> crate::Result<()> {
        use crate::unix_async::*;

        let socket_path = get_socket_path(SocketType::Listener)?;
        let mut stream = UnixStream::connect(socket_path).await?;

        let mut active_windows = vec![];
        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf).await?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];
            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed {
                self.event_primer_async(event, &mut active_windows).await?;
            }
        }

        Ok(())
    }
}
