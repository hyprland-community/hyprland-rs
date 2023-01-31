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
        use crate::{
            data::{FullscreenState, Monitors, Workspace},
            prelude::*,
        };
        EventListener {
            events: init_events!(),
            state: State {
                active_workspace: match Workspace::get_active() {
                    Ok(work) => WorkspaceType::Regular(work.id.to_string()),
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
                active_monitor: match Monitors::get() {
                    Ok(mut monitors) => match monitors.find(|item| item.focused) {
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

    mut_add_listener!(workspace_change d, WorkspaceType, "on workspace change", "changed workspace to" => id);
    mut_add_listener!(workspace_added, WorkspaceType, "a workspace is created", "workspace was added" => id);
    mut_add_listener!(workspace_destroy ed, WorkspaceType, "a workspace is destroyed", "workspace was destroyed" => id);
    mut_add_listener!(workspace_moved, MonitorEventData, "a workspace is moved", "workspace was moved" => id);
    mut_add_listener!(active_monitor_change d, MonitorEventData, "the active monitor is changed", "Active monitor changed to" => data);
    mut_add_listener!(active_window_change d, Option<WindowEventData>, "the active window is changed", "Active window changed" => data);
    mut_add_listener!(fullscreen_state_change d, bool, "the active monitor is changed", "Fullscreen is on" => state);
    mut_add_listener!(monitor_added, String, "a new monitor is added", "Monitor added" => data);
    mut_add_listener!(monitor_removed, String, "a monitor is removed", "Monitor removed" => data);
    mut_add_listener!(window_open, WindowOpenEvent, "a window is opened", "Window opened" => data);
    mut_add_listener!(window_close, Address, "a window is closed", "Window closed" => data);
    mut_add_listener!(window_moved, WindowMoveEvent, "a window is moved", "Window moved" => data);
    mut_add_listener!(keyboard_layout_change, LayoutEvent, "the keyboard layout is changed", "Layout changed" => data);
    mut_add_listener!(sub_map_change d, String, "the sub map is changed", "Submap changed" => data);
    mut_add_listener!(layer_open, String, "a new layer is opened", "Layer opened" => data);
    mut_add_listener!(layer_closed, String, "a layer is closed", "Layer closed" => data);
    mut_add_listener!(float_state, WindowFloatEventData, "the float state of a window is changed", "Float state changed" => data);
    mut_add_listener!(urgent_state, Address, "the urgent state of a window is changed", "urgent state changed" => data);

    async fn event_executor(&mut self, event: &Event) -> HResult<()> {
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
            Event::ActiveMonitorChanged(even) => mut_state_arm!(
                even.clone(),
                active_monitor_changed_events,
                active_monitor,
                even.0.clone(),
                self
            ),
            Event::ActiveWindowChanged(Some(even)) => {
                mut_arm!(Some(even.clone()), active_window_changed_events, self)
            }
            Event::ActiveWindowChanged(None) => mut_arm!(None, active_window_changed_events, self),
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
        }
        Ok(())
    }

    fn event_executor_sync(&mut self, event: &Event) -> HResult<()> {
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
            Event::ActiveMonitorChanged(MonitorEventData(monitor, id)) => {
                mut_state_arm_sync!(
                    MonitorEventData(monitor.clone(), id.clone()),
                    active_monitor_changed_events,
                    active_monitor,
                    monitor.clone(),
                    self
                )
            }
            Event::ActiveWindowChanged(Some(WindowEventData(class, title))) => {
                mut_arm_sync!(
                    Some(WindowEventData(class.clone(), title.clone())),
                    active_window_changed_events,
                    self
                )
            }
            Event::ActiveWindowChanged(None) => {
                mut_arm_sync!(None, active_window_changed_events, self)
            }
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
    pub async fn start_listener_async(&mut self) -> HResult<()> {
        use crate::unix_async::*;

        let socket_path = get_socket_path(SocketType::Listener);

        let mut stream = UnixStream::connect(socket_path).await?;

        let mut buf = [0; 4096];

        loop {
            let num_read = stream.read(&mut buf).await?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];

            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

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
    /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id:?}"));
    /// listener.start_listener();
    /// ```
    pub fn start_listener(mut self) -> HResult<()> {
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

            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed.iter() {
                self.event_executor_sync(event)?;
            }
        }

        Ok(())
    }
}
