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
pub struct AsyncMutableEventListener {
    pub(crate) events: AsyncEvents,
    pub(crate) state: StateV2,
}

// Mark the EventListener as thread-safe
unsafe impl Send for AsyncMutableEventListener {}
unsafe impl Sync for AsyncMutableEventListener {}

impl Default for AsyncMutableEventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncMutableEventListener {
    /// This method creates a new EventListener instance
    ///
    /// ```rust
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> Self {
        Self {
            events: init_events!(AsyncEvents),
            state: StateV2::new("", "", false),
        }
    }

    async fn event_executor(&mut self, event: &Event) {
        match event {
            Event::WorkspaceChanged(id) => {
                arm_async_mut!(c id, workspace_changed_events, workspace, self)
            }
            Event::WorkspaceAdded(id) => arm_async_mut!(id, workspace_added_events, self),
            Event::WorkspaceDeleted(id) => arm_async_mut!(id, workspace_destroyed_events, self),
            Event::WorkspaceMoved(work) => arm_async_mut!(work, workspace_moved_events, self),
            Event::ActiveMonitorChanged(mon) => {
                arm_async_mut!(cv mon, active_monitor_changed_events, monitor, &mon.0, self)
            }
            Event::ActiveWindowChangedMerged(event) => {
                arm_async_mut!(event, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => {
                arm_async_mut!(
                    cv bool,
                    fullscreen_state_changed_events,
                    fullscreen,
                    *bool,
                    self
                )
            }
            Event::MonitorAdded(monitor) => arm_async_mut!(monitor, monitor_added_events, self),
            Event::MonitorRemoved(monitor) => arm_async_mut!(monitor, monitor_removed_events, self),
            Event::WindowClosed(addr) => arm_async_mut!(addr, window_close_events, self),
            Event::WindowMoved(win) => arm_async_mut!(win, window_moved_events, self),
            Event::WindowOpened(win) => arm_async_mut!(win, window_open_events, self),
            Event::LayoutChanged(lay) => arm_async_mut!(lay, keyboard_layout_change_events, self),
            Event::SubMapChanged(map) => arm_async_mut!(map, sub_map_changed_events, self),
            Event::LayerOpened(namespace) => arm_async_mut!(namespace, layer_open_events, self),
            Event::LayerClosed(namespace) => arm_async_mut!(namespace, layer_closed_events, self),
            Event::FloatStateChanged(state) => arm_async_mut!(state, float_state_events, self),
            Event::UrgentStateChanged(state) => arm_async_mut!(state, urgent_state_events, self),
        }
    }

    async fn event_primer(&mut self, event: &Event, active: &mut Option<Option<(String, String)>>) {
        if let Event::ActiveWindowChangedV1(event) = event {
            *active = Some(event.clone());
        } else if let Event::ActiveWindowChangedV2(Some(addr)) = event {
            match active {
                Some(Some((class, title))) => {
                    self.event_executor(&Event::ActiveWindowChangedMerged(Some(WindowEventData(
                        class.to_string(),
                        title.to_string(),
                        addr.clone(),
                    ))))
                    .await;
                }
                Some(None) => {}
                None => {}
            };
        } else if let Event::ActiveWindowChangedV2(None) = event {
            self.event_executor(&Event::ActiveWindowChangedMerged(None))
                .await;
        } else {
            self.event_executor(event).await;
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
    pub async fn start_listener_async(&mut self) -> HResult<()> {
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
                self.event_primer(event, &mut active_window_buf).await;
            }
        }

        Ok(())
    }
}
