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

// Mark the EventListener as thread-safe
#[allow(unsafe_code)]
unsafe impl Send for EventListener {}
#[allow(unsafe_code)]
unsafe impl Sync for EventListener {}

impl Default for EventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl HasExecutor for EventListener {
    fn event_executor(&mut self, event: &Event) -> crate::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => mut_state_arm_sync!(
                id.clone(),
                workspace_changed_events,
                active_workspace,
                id.clone(),
                self
            ),
            Event::WorkspaceAdded(id) => mut_arm_sync!(id.clone(), workspace_added_events, self),
            Event::WorkspaceDeleted(id) => {
                mut_arm_sync!(id.clone(), workspace_destroyed_events, self)
            }
            Event::WorkspaceMoved(id) => mut_arm_sync!(id.clone(), workspace_moved_events, self),
            Event::WorkspaceRename(even) => {
                mut_arm_sync!(even.clone(), workspace_rename_events, self)
            }
            Event::ActiveMonitorChanged(MonitorEventData {
                monitor_name,
                workspace,
            }) => {
                mut_state_arm_sync!(
                    MonitorEventData {
                        monitor_name: monitor_name.clone(),
                        workspace: workspace.clone()
                    },
                    active_monitor_changed_events,
                    active_monitor,
                    monitor_name.clone(),
                    self
                )
            }
            Event::ActiveWindowChangedMerged(Some(event)) => {
                mut_arm_sync!(Some(event.clone()), active_window_changed_events, self)
            }
            Event::ActiveWindowChangedMerged(None) => {
                mut_arm_sync!(None, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => mut_state_arm_sync!(
                *bool,
                fullscreen_state_changed_events,
                fullscreen_state,
                *bool,
                self
            ),
            Event::MonitorAdded(monitor) => {
                mut_arm_sync!(monitor.clone(), monitor_added_events, self)
            }
            Event::MonitorRemoved(monitor) => {
                mut_arm_sync!(monitor.clone(), monitor_removed_events, self)
            }
            Event::WindowClosed(addr) => mut_arm_sync!(addr.clone(), window_close_events, self),
            Event::WindowMoved(even) => mut_arm_sync!(even.clone(), window_moved_events, self),
            Event::WindowOpened(even) => mut_arm_sync!(even.clone(), window_open_events, self),
            Event::LayoutChanged(lay) => {
                mut_arm_sync!(lay.clone(), keyboard_layout_change_events, self)
            }
            Event::SubMapChanged(even) => mut_arm_sync!(even.clone(), sub_map_changed_events, self),
            Event::LayerOpened(even) => mut_arm_sync!(even.clone(), layer_open_events, self),
            Event::LayerClosed(even) => mut_arm_sync!(even.clone(), layer_closed_events, self),
            Event::FloatStateChanged(even) => {
                mut_arm_sync!(even.clone(), float_state_events, self)
            }
            Event::UrgentStateChanged(even) => {
                mut_arm_sync!(even.clone(), urgent_state_events, self)
            }
            Event::Minimize(data) => mut_arm_sync!(data.clone(), minimize_events, self),
            Event::WindowTitleChanged(addr) => {
                mut_arm_sync!(addr.clone(), window_title_changed_events, self)
            }
            Event::Screencast(data) => mut_arm_sync!(*data, screencast_events, self),
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
    pub fn new() -> EventListener {
        use crate::{
            data::{FullscreenState, Monitors, Workspace},
            prelude::*,
        };
        EventListener {
            events: init_events!(Events),
            state: State {
                active_workspace: match Workspace::get_active() {
                    Ok(work) => WorkspaceType::Regular(work.id.to_string()),
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
                active_monitor: match Monitors::get() {
                    Ok(monitors) => match monitors.into_iter().find(|item| item.focused) {
                        Some(mon) => mon.name,
                        None => panic!("No active monitor?"),
                    },
                    Err(e) => panic!("A error occured when parsing json with serde {e}"),
                },
                fullscreen_state: match FullscreenState::get() {
                    Ok(fstate) => fstate.bool(),
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
            },
        }
    }

    pub(crate) async fn event_executor_async(&mut self, event: &Event) -> crate::Result<()> {
        match event {
            Event::WorkspaceChanged(id) => mut_state_arm!(
                id.clone(),
                workspace_changed_events,
                active_workspace,
                id.clone(),
                self
            ),
            Event::WorkspaceAdded(id) => mut_arm!(id.clone(), workspace_added_events, self),
            Event::WorkspaceDeleted(id) => mut_arm!(id.clone(), workspace_destroyed_events, self),
            Event::WorkspaceMoved(id) => mut_arm!(id.clone(), workspace_moved_events, self),
            Event::WorkspaceRename(even) => mut_arm!(even.clone(), workspace_rename_events, self),
            Event::ActiveMonitorChanged(even) => mut_state_arm!(
                even.clone(),
                active_monitor_changed_events,
                active_monitor,
                even.monitor_name.clone(),
                self
            ),
            Event::ActiveWindowChangedMerged(Some(event)) => {
                mut_arm!(Some(event.clone()), active_window_changed_events, self)
            }
            Event::ActiveWindowChangedMerged(None) => {
                mut_arm!(None, active_window_changed_events, self)
            }
            Event::ActiveWindowChangedV1(_) => (),
            Event::ActiveWindowChangedV2(_) => (),
            Event::FullscreenStateChanged(bool) => mut_state_arm!(
                *bool,
                fullscreen_state_changed_events,
                fullscreen_state,
                *bool,
                self
            ),
            Event::MonitorAdded(monitor) => mut_arm!(monitor.clone(), monitor_added_events, self),
            Event::MonitorRemoved(monitor) => {
                mut_arm!(monitor.clone(), monitor_removed_events, self)
            }
            Event::WindowClosed(addr) => mut_arm!(addr.clone(), window_close_events, self),
            Event::WindowMoved(even) => mut_arm!(even.clone(), window_moved_events, self),
            Event::WindowOpened(even) => mut_arm!(even.clone(), window_open_events, self),
            Event::LayoutChanged(lay) => mut_arm!(lay.clone(), keyboard_layout_change_events, self),
            Event::SubMapChanged(map) => mut_arm!(map.clone(), sub_map_changed_events, self),
            Event::LayerOpened(even) => mut_arm!(even.clone(), layer_open_events, self),
            Event::LayerClosed(even) => mut_arm!(even.clone(), layer_closed_events, self),
            Event::FloatStateChanged(even) => mut_arm!(even.clone(), float_state_events, self),
            Event::UrgentStateChanged(even) => mut_arm!(even.clone(), urgent_state_events, self),
            Event::Minimize(data) => mut_arm!(data.clone(), minimize_events, self),
            Event::Screencast(data) => mut_arm!(*data, screencast_events, self),
            Event::WindowTitleChanged(addr) => {
                mut_arm!(addr.clone(), window_title_changed_events, self)
            }
        }
        Ok(())
    }

    async fn event_primer_async(
        &mut self,
        event: &Event,
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
            for (index, awin) in abuf.iter_mut().enumerate() {
                if awin.title.is_empty() && awin.class.is_empty() {
                    awin.class = data.clone().map(|i| i.0).into();
                    awin.title = data.clone().map(|i| i.1).into();
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

        let socket_path = get_socket_path(SocketType::Listener);

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

            for event in parsed.iter() {
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

        let socket_path = get_socket_path(SocketType::Listener);

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

            for event in parsed.iter() {
                self.event_primer(event, &mut active_windows)?;
            }
        }

        Ok(())
    }
}
