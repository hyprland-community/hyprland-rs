use crate::shared::*;
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
            events: Events {
                workspace_changed_events: vec![],
                workspace_added_events: vec![],
                workspace_destroyed_events: vec![],
                active_monitor_changed_events: vec![],
                active_window_changed_events: vec![],
                fullscreen_state_changed_events: vec![],
                monitor_removed_events: vec![],
                monitor_added_events: vec![],
                window_open_events: vec![],
                window_close_events: vec![],
                window_moved_events: vec![],
                keyboard_layout_change_events: vec![],
                layer_open_events: vec![],
                layer_closed_events: vec![],
                sub_map_changed_events: vec![],
                workspace_moved_events: vec![],
                float_state_events: vec![],
            },
            state: State {
                active_workspace: match Workspace::get_active() {
                    Ok(work) => work.id,
                    Err(e) => panic!("Error parsing data whith serde: {e}"),
                },
                active_monitor: match Monitors::get() {
                    Ok(monitors) => match monitors.collect().iter().find(|item| item.focused) {
                        Some(mon) => mon.name.clone(),
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

    // /// This method adds a event to the listener which executes on workspace change
    // ///
    // /// ```rust, no_run
    // /// use hyprland::event_listener::EventListenerMutable as EventListener;
    // /// let mut listener = EventListener::new();
    // /// listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id:?}"));
    // /// listener.start_listener_blocking();
    // /// ```
    // pub fn add_workspace_change_handler(
    //     &mut self,
    //     f: impl Fn(WorkspaceType, &mut State) + 'static,
    // ) {
    //     self.events
    //         .workspace_changed_events
    //         .push(EventTypes::MutableState(Box::new(f)));
    // }
    mut_add_listener!(
        reg add_workspace_change_handler,
        workspace_changed_events,
        WorkspaceType,
        "This method adds a event to the listener which executes on workspace change",
        r#"listener.add_workspace_change_handler(|id, _| println!("changed workspace to {id:?}"));"#
    );

    mut_add_listener!(
        reg add_workspace_added_handler,
        workspace_added_events,
        WorkspaceType,
        "This method adds a event to the listener which executes when a new workspace is created",
        r#"listener.add_workspace_added_handler(|id, _| println!("workspace {id:?} was added"));"#
    );

    mut_add_listener!(
        reg add_workspace_destroy_handler,
        workspace_destroyed_events,
        WorkspaceType,
        "This method adds a event to the listener which executes when a workspace is destroyed",
        r#"listener.add_workspace_destroy_handler(|id, _| println!("workspace {id:?} was destroyed"));"#
    );

    mut_add_listener!(
        reg add_workspace_moved_handler,
        workspace_moved_events,
        MonitorEventData,
        "This method to add a event to the listener which executes when a workspace is moved",
        r#"listener.add_workspace_moved_handler(|id, _| println!("workspace {id:?} was moved"));"#
    );

    mut_add_listener!(
        reg add_active_monitor_change_handler,
        active_monitor_changed_events,
        MonitorEventData,
        "This method adds a event to the listener which executes when the active monitor is changed",
        r#"listener.add_active_monitor_change_handler(|data, _| println!("Active Monitor changed: {data:#?}"));"#
    );

    mut_add_listener!(
        reg add_active_window_change_handler,
        active_window_changed_events,
        Option<WindowEventData>,
        "This method adds a event to the listener which executes when the active window is changed",
        r#"listener.add_active_window_change_handler(|data, _| println!("Active window changed: {data:#?}"));"#
    );

    mut_add_listener!(
        reg add_fullscreen_state_change_handler,
        fullscreen_state_changed_events,
        bool,
        "This method adds a event to the listener which executes when the fullscreen state is changed",
        r#"listener.add_fullscreen_state_change_handler(|state, _| println!("Fullscreen is on: {state}"));"#
    );

    mut_add_listener!(
        reg add_monitor_added_handler,
        monitor_added_events,
        String,
        "This method adds a event to the listener which executes when a new monitor is added",
        r#"listener.add_monitor_added_handler(|data, _| println!("Monitor added: {data}"));"#
    );

    mut_add_listener!(
        reg add_monitor_removed_handler,
        monitor_removed_events,
        String,
        "This method adds a event to the listener which executes when a monitor is removed",
        r#"listener.add_monitor_removed_handler(|data, _| println!("Monitor removed: {data}"));"#
    );

    mut_add_listener!(
        reg add_keyboard_layout_change_handler,
        keyboard_layout_change_events,
        LayoutEvent,
        "This method adds a event to the listener which executes when the keyboard layout is changed",
        r#"listener.add_keyboard_layout_change_handler(|data, _| println!("Keyboard Layout Changed: {data:#?}"));"#
    );

    mut_add_listener!(
        reg add_sub_map_change_handler,
        sub_map_changed_events,
        String,
        "This method adds a event to the listener which executes when the submap is changed",
        r#"listener.add_sub_map_change_handler(|data, _| println!("Submap changed: {data}"));"#
    );

    mut_add_listener!(
        reg add_window_open_handler,
        window_open_events,
        WindowOpenEvent,
        "This method adds an event to the listener which executes when a window is opened",
        r#"listener.add_window_open_handler(|data, _| println!("Window opened: {data:#?}"));"#
    );

    mut_add_listener!(
        reg add_window_close_handler,
        window_close_events,
        Address,
        "This method adds an event to the listener which executes when a window is closed",
        r#"listener.add_window_close_handler(|data, _| println!("Window closed: {data}"));"#
    );

    mut_add_listener!(
        reg add_window_moved_handler,
        window_moved_events,
        WindowMoveEvent,
        "This method adds an event to the listener which executes when a window is moved",
        r#"listener.add_window_moved_handler(|data, _| println!("Window moved: {data:#?}"));"#
    );

    mut_add_listener!(
        reg add_layer_open_handler,
        layer_open_events,
        String,
        "This method adds an event to the listener which executes when a new layer is opened",
        r#"listener.add_layer_open_handler(|data, _| println!("Layer opened: {data}"));"#
    );

    mut_add_listener!(
        reg add_layer_closed_handler,
        layer_closed_events,
        String,
        "This method adds an event to the listener which executes when a layer is closed",
        r#"listener.add_layer_closed_handler(|data, _| println!("Layer closed: {data}"));"#
    );

    mut_add_listener!(
        reg add_float_state_handler,
        float_state_events,
        WindowFloatEventData,
        "This method adds an event to the listener which executes when the float state of a window is changed",
        r#"listener.add_float_state_handler(|data, _| println!("Float state changed: {data:#?}"));"#
    );

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
    }
}
