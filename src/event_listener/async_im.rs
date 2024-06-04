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
        use Event::*;
        match event {
            WorkspaceChanged(id) => arm_async!(id, workspace_changed_events, self),
            WorkspaceAdded(id) => arm_async!(id, workspace_added_events, self),
            WorkspaceDeleted(data) => arm_async!(data, workspace_destroyed_events, self),
            WorkspaceMoved(evend) => arm_async!(evend, workspace_moved_events, self),
            WorkspaceRename(even) => arm_async!(even, workspace_rename_events, self),
            ActiveMonitorChanged(evend) => arm_async!(evend, active_monitor_changed_events, self),
            ActiveWindowChangedMerged(event) => {
                arm_async!(event, active_window_changed_events, self)
            }
            ActiveWindowChangedV1(_) => (),
            ActiveWindowChangedV2(_) => (),
            FullscreenStateChanged(bool) => arm_async!(bool, fullscreen_state_changed_events, self),
            MonitorAdded(monitor) => arm_async!(monitor, monitor_added_events, self),
            MonitorRemoved(monitor) => arm_async!(monitor, monitor_removed_events, self),
            WindowClosed(addr) => arm_async!(addr, window_close_events, self),
            WindowMoved(even) => arm_async!(even, window_moved_events, self),
            WindowOpened(even) => arm_async!(even, window_open_events, self),
            SpecialRemoved(monitor) => arm_async!(monitor, special_removed_events, self),
            ChangedSpecial(data) => arm_async!(data, special_changed_events, self),
            LayoutChanged(even) => arm_async!(even, keyboard_layout_change_events, self),
            SubMapChanged(map) => arm_async!(map, sub_map_changed_events, self),
            LayerOpened(namespace) => arm_async!(namespace, layer_open_events, self),
            LayerClosed(namespace) => arm_async!(namespace, layer_closed_events, self),
            FloatStateChanged(even) => arm_async!(even, float_state_events, self),
            UrgentStateChanged(even) => arm_async!(even, urgent_state_events, self),
            Minimize(data) => arm_async!(data, minimize_events, self),
            WindowTitleChanged(addr) => arm_async!(addr, window_title_changed_events, self),
            Screencast(data) => arm_async!(data, screencast_events, self),
            ConfigReloaded => arm_async!(config_reloaded_events, self),
            IgnoreGroupLockStateChanged(bool) => {
                arm_async!(bool, ignore_group_lock_state_changed_events, self)
            }
            LockGroupsStateChanged(bool) => {
                arm_async!(bool, lock_groups_state_changed_events, self)
            }
            WindowPinned(data) => arm_async!(data, window_pin_state_toggled_events, self),
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
