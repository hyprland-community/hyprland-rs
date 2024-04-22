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
/// use hyprland::event_listener::EventListenerMutable as EventListener;
/// let mut listener = EventListener::new(); // creates a new listener
/// // add a event handler which will be ran when this event happens
/// listener.add_workspace_change_handler(|data, _| println!("{:#?}", data));
/// listener.start_listener(); // or `.start_listener_async().await` if async
/// ```
pub struct EventListener {
    pub(crate) events: Events,
    /// The state of some of the events
    pub state: State,
}

impl HasExecutor for EventListener {
    fn event_executor(&mut self, event: Event) -> crate::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => mut_state_arm_sync!(
                id,
                workspace_changed_events,
                active_workspace,
                id.clone(),
                self
            ),
            Event::WorkspaceAdded(id) => mut_arm_sync!(id, workspace_added_events, self),
            Event::WorkspaceDeleted(id) => mut_arm_sync!(id, workspace_destroyed_events, self),
            Event::WorkspaceMoved(id) => mut_arm_sync!(id, workspace_moved_events, self),
            Event::WorkspaceRename(even) => mut_arm_sync!(even, workspace_rename_events, self),
            Event::ActiveMonitorChanged(monitor) => {
                mut_state_arm_sync!(
                    monitor,
                    active_monitor_changed_events,
                    active_monitor,
                    monitor.monitor_name.clone(),
                    self
                )
            }
            Event::ActiveWindowChangedMerged(event) => {
                mut_arm_sync!(event, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => mut_state_arm_sync!(
                bool,
                fullscreen_state_changed_events,
                fullscreen_state,
                bool,
                self
            ),
            Event::MonitorAdded(monitor) => mut_arm_sync!(monitor, monitor_added_events, self),
            Event::MonitorRemoved(monitor) => mut_arm_sync!(monitor, monitor_removed_events, self),
            Event::WindowClosed(addr) => mut_arm_sync!(addr, window_close_events, self),
            Event::WindowMoved(even) => mut_arm_sync!(even, window_moved_events, self),
            Event::WindowOpened(even) => mut_arm_sync!(even, window_open_events, self),
            Event::LayoutChanged(lay) => mut_arm_sync!(lay, keyboard_layout_change_events, self),
            Event::SubMapChanged(even) => mut_arm_sync!(even, sub_map_changed_events, self),
            Event::LayerOpened(even) => mut_arm_sync!(even, layer_open_events, self),
            Event::LayerClosed(even) => mut_arm_sync!(even, layer_closed_events, self),
            Event::FloatStateChanged(even) => mut_arm_sync!(even, float_state_events, self),
            Event::UrgentStateChanged(even) => mut_arm_sync!(even, urgent_state_events, self),
            Event::Minimize(data) => mut_arm_sync!(data, minimize_events, self),
            Event::WindowTitleChanged(addr) => {
                mut_arm_sync!(addr, window_title_changed_events, self)
            }
            Event::Screencast(data) => mut_arm_sync!(data, screencast_events, self),
        }
        Ok(())
    }
}

impl EventListener {
    /// This method creates a new EventListener instance
    ///
    /// ```rust
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> crate::Result<EventListener> {
        use crate::{
            data::{FullscreenState, Monitors, Workspace},
            prelude::*,
        };
        Ok(EventListener {
            events: init_events!(Events),
            state: State {
                active_workspace: WorkspaceType::Regular(Workspace::get_active()?.id.to_string()),
                active_monitor: match Monitors::get()?.into_iter().find(|item| item.focused) {
                    Some(mon) => mon.name,
                    None => hypr_err!("No active Hyprland monitor detected!"),
                },
                fullscreen_state: FullscreenState::get()?.bool(),
            },
        })
    }

    pub(crate) async fn event_executor_async(&mut self, event: Event) -> crate::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => mut_state_arm!(
                id,
                workspace_changed_events,
                active_workspace,
                id.clone(),
                self
            ),
            Event::WorkspaceAdded(id) => mut_arm!(id, workspace_added_events, self),
            Event::WorkspaceDeleted(id) => mut_arm!(id, workspace_destroyed_events, self),
            Event::WorkspaceMoved(id) => mut_arm!(id, workspace_moved_events, self),
            Event::WorkspaceRename(even) => mut_arm!(even, workspace_rename_events, self),
            Event::ActiveMonitorChanged(even) => mut_state_arm!(
                even,
                active_monitor_changed_events,
                active_monitor,
                even.monitor_name.clone(),
                self
            ),
            Event::ActiveWindowChangedMerged(event) => {
                mut_arm!(event, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => mut_state_arm!(
                bool,
                fullscreen_state_changed_events,
                fullscreen_state,
                bool,
                self
            ),
            Event::MonitorAdded(monitor) => mut_arm!(monitor, monitor_added_events, self),
            Event::MonitorRemoved(monitor) => mut_arm!(monitor, monitor_removed_events, self),
            Event::WindowClosed(addr) => mut_arm!(addr, window_close_events, self),
            Event::WindowMoved(even) => mut_arm!(even, window_moved_events, self),
            Event::WindowOpened(even) => mut_arm!(even, window_open_events, self),
            Event::LayoutChanged(lay) => mut_arm!(lay, keyboard_layout_change_events, self),
            Event::SubMapChanged(map) => mut_arm!(map, sub_map_changed_events, self),
            Event::LayerOpened(even) => mut_arm!(even, layer_open_events, self),
            Event::LayerClosed(even) => mut_arm!(even, layer_closed_events, self),
            Event::FloatStateChanged(even) => mut_arm!(even, float_state_events, self),
            Event::UrgentStateChanged(even) => mut_arm!(even, urgent_state_events, self),
            Event::Minimize(data) => mut_arm!(data, minimize_events, self),
            Event::Screencast(data) => mut_arm!(data, screencast_events, self),
            Event::WindowTitleChanged(addr) => mut_arm!(addr, window_title_changed_events, self),
        }
        Ok(())
    }

    async fn event_primer_async(
        &mut self,
        event: Event,
        abuf: &mut Vec<ActiveWindowState>,
    ) -> crate::Result<()>
    where
        Self: std::marker::Sized,
    {
        if abuf.is_empty() {
            abuf.push(ActiveWindowState::new());
        }
        if let Event::ActiveWindowChangedV1(data) = event {
            let mut to_remove = vec![];
            let data = into(data);
            for (index, awin) in abuf.iter_mut().enumerate() {
                if awin.title.is_empty() && awin.class.is_empty() {
                    (awin.class, awin.title) = data.clone();
                }
                if awin.ready() {
                    awin.execute_async_mut(self).await?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove {
                abuf.remove(index);
            }
        } else if let Event::ActiveWindowChangedV2(data) = event {
            let mut to_remove = vec![];
            for (index, awin) in abuf.iter_mut().enumerate() {
                if awin.addr.is_empty() {
                    awin.addr = data.clone().into();
                }
                if awin.ready() {
                    awin.execute_async_mut(self).await?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove {
                abuf.remove(index);
            }
        } else {
            self.event_executor_async(event).await?;
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
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id:?}"));
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

    /// This method starts the event listener (blocking)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListenerMutable as EventListener;
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id:?}"));
    /// listener.start_listener();
    /// ```
    pub fn start_listener(mut self) -> crate::Result<()> {
        use io::prelude::*;
        use std::os::unix::net::UnixStream;

        let socket_path = get_socket_path(SocketType::Listener)?;

        let mut stream = UnixStream::connect(socket_path)?;

        let mut active_windows = vec![];

        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];

            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed {
                self.event_primer(event, &mut active_windows)?;
            }
        }

        Ok(())
    }
}
