use crate::shared::*;
use once_cell::sync::Lazy;
use regex::{Error as RegexError, Regex};
use std::fmt::Debug;
use std::io;
use std::pin::Pin;

/// This trait provides shared behaviour for listener types
pub(crate) trait Listener: HasExecutor {
    /// This method starts the event listener
    fn start_listener() -> crate::Result<()>;
}

/// This trait provides shared behaviour for listener types
pub(crate) trait AsyncListener: HasAsyncExecutor {
    /// This method starts the event listener (async)
    async fn start_listener_async() -> crate::Result<()>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ActiveWindowValue<T> {
    Queued(T), // aka Some(T)
    None,      // No current window
    Empty,     // Empty queue
}

impl<T> ActiveWindowValue<T> {
    pub fn reset(&mut self) {
        *self = Self::Empty;
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ActiveWindowState {
    pub class: ActiveWindowValue<String>,
    pub title: ActiveWindowValue<String>,
    pub addr: ActiveWindowValue<Address>,
}

pub(crate) trait HasExecutor {
    fn event_executor(&mut self, event: Event) -> crate::Result<()>;

    fn event_primer(&mut self, event: Event, abuf: &mut Vec<ActiveWindowState>) -> crate::Result<()>
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
                    awin.execute(self)?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove.into_iter().rev() {
                abuf.swap_remove(index);
            }
        } else if let Event::ActiveWindowChangedV2(data) = event {
            let mut to_remove = vec![];
            for (index, awin) in abuf.iter_mut().enumerate() {
                if awin.addr.is_empty() {
                    awin.addr = data.clone().into();
                }
                if awin.ready() {
                    awin.execute(self)?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove.into_iter().rev() {
                abuf.swap_remove(index);
            }
        } else {
            self.event_executor(event)?;
        }
        Ok(())
    }
}

pub(crate) trait HasAsyncExecutor {
    async fn event_executor_async(&mut self, event: Event) -> crate::Result<()>;

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
                    awin.execute_async(self).await?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove.into_iter().rev() {
                abuf.swap_remove(index);
            }
        } else if let Event::ActiveWindowChangedV2(data) = event {
            let mut to_remove = vec![];
            for (index, awin) in abuf.iter_mut().enumerate() {
                if awin.addr.is_empty() {
                    awin.addr = data.clone().into();
                }
                if awin.ready() {
                    awin.execute_async(self).await?;
                    to_remove.push(index);
                    break;
                }
            }
            for index in to_remove.into_iter().rev() {
                abuf.swap_remove(index);
            }
        } else {
            self.event_executor_async(event).await?;
        }
        Ok(())
    }
}

impl ActiveWindowState {
    pub fn execute<T: HasExecutor>(&mut self, listener: &mut T) -> crate::Result<()> {
        use ActiveWindowValue::{None, Queued};
        let data = (&self.title, &self.class, &self.addr);
        if let (Queued(ref title), Queued(ref class), Queued(ref addr)) = data {
            listener.event_executor(Event::ActiveWindowChangedMerged(Some(WindowEventData {
                window_class: class.to_string(),
                window_title: title.to_string(),
                window_address: addr.clone(),
            })))?;
            self.reset();
        } else if let (None, None, None) = data {
            listener.event_executor(Event::ActiveWindowChangedMerged(Option::None))?;
        }
        Ok(())
    }
    pub async fn execute_async<T: HasAsyncExecutor>(
        &mut self,
        listener: &mut T,
    ) -> crate::Result<()> {
        use ActiveWindowValue::{None, Queued};
        let data = (&self.title, &self.class, &self.addr);
        if let (Queued(ref title), Queued(ref class), Queued(ref addr)) = data {
            listener
                .event_executor_async(Event::ActiveWindowChangedMerged(Some(WindowEventData {
                    window_class: class.to_string(),
                    window_title: title.to_string(),
                    window_address: addr.clone(),
                })))
                .await?;
            self.reset();
        } else if let (None, None, None) = data {
            listener
                .event_executor_async(Event::ActiveWindowChangedMerged(Option::None))
                .await?;
        }
        Ok(())
    }
    pub async fn execute_async_mut(
        &mut self,
        listener: &mut super::EventListenerMutable,
    ) -> crate::Result<()> {
        use ActiveWindowValue::{None, Queued};
        let data = (&self.title, &self.class, &self.addr);
        if let (Queued(ref title), Queued(ref class), Queued(ref addr)) = data {
            listener
                .event_executor_async(Event::ActiveWindowChangedMerged(Some(WindowEventData {
                    window_class: class.to_string(),
                    window_title: title.to_string(),
                    window_address: addr.clone(),
                })))
                .await?;
            self.reset();
        } else if let (None, None, None) = data {
            listener
                .event_executor_async(Event::ActiveWindowChangedMerged(Option::None))
                .await?;
        }
        Ok(())
    }

    pub fn ready(&self) -> bool {
        !self.class.is_empty() && !self.title.is_empty() && !self.addr.is_empty()
    }
    pub fn reset(&mut self) {
        self.class.reset();
        self.title.reset();
        self.addr.reset();
    }
    pub fn new() -> Self {
        Self {
            class: ActiveWindowValue::Empty,
            title: ActiveWindowValue::Empty,
            addr: ActiveWindowValue::Empty,
        }
    }
}

impl<T> From<Option<T>> for ActiveWindowValue<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => ActiveWindowValue::Queued(v),
            None => ActiveWindowValue::None,
        }
    }
}

pub(crate) fn into<T>(from: Option<(T, T)>) -> (ActiveWindowValue<T>, ActiveWindowValue<T>) {
    if let Some((first, second)) = from {
        (
            ActiveWindowValue::Queued(first),
            ActiveWindowValue::Queued(second),
        )
    } else {
        (ActiveWindowValue::None, ActiveWindowValue::None)
    }
}

pub(crate) enum EventTypes<T: ?Sized, U: ?Sized> {
    MutableState(Box<U>),
    Regular(Box<T>),
}

pub(crate) enum AsyncEventTypes<T: ?Sized, U: ?Sized> {
    #[allow(dead_code)]
    MutableState(Pin<Box<U>>),
    Regular(Pin<Box<T>>),
}

pub(crate) type VoidFuture = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
pub(crate) type VoidFutureMut =
    std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>;

pub(crate) type Closure<T> = EventTypes<dyn Fn(T), dyn Fn(T, &mut State)>;
pub(crate) type AsyncClosure<T> = AsyncEventTypes<
    dyn Sync + Send + Fn(T) -> VoidFuture,
    dyn Sync + Send + Fn(T, &mut State) -> VoidFutureMut,
>;
pub(crate) type Closures<T> = Vec<Closure<T>>;
pub(crate) type AsyncClosures<T> = Vec<AsyncClosure<T>>;

#[allow(clippy::type_complexity)]
pub(crate) struct Events {
    pub(crate) workspace_changed_events: Closures<WorkspaceType>,
    pub(crate) workspace_added_events: Closures<WorkspaceType>,
    pub(crate) workspace_destroyed_events: Closures<WorkspaceType>,
    pub(crate) workspace_moved_events: Closures<MonitorEventData>,
    pub(crate) workspace_rename_events: Closures<WorkspaceRenameEventData>,
    pub(crate) active_monitor_changed_events: Closures<MonitorEventData>,
    pub(crate) active_window_changed_events: Closures<Option<WindowEventData>>,
    pub(crate) fullscreen_state_changed_events: Closures<bool>,
    pub(crate) monitor_removed_events: Closures<String>,
    pub(crate) monitor_added_events: Closures<String>,
    pub(crate) keyboard_layout_change_events: Closures<LayoutEvent>,
    pub(crate) sub_map_changed_events: Closures<String>,
    pub(crate) window_open_events: Closures<WindowOpenEvent>,
    pub(crate) window_close_events: Closures<Address>,
    pub(crate) window_moved_events: Closures<WindowMoveEvent>,
    pub(crate) layer_open_events: Closures<String>,
    pub(crate) layer_closed_events: Closures<String>,
    pub(crate) float_state_events: Closures<WindowFloatEventData>,
    pub(crate) urgent_state_events: Closures<Address>,
    pub(crate) minimize_events: Closures<MinimizeEventData>,
    pub(crate) window_title_changed_events: Closures<Address>,
    pub(crate) screencast_events: Closures<ScreencastEventData>,
}

#[allow(clippy::type_complexity)]
pub(crate) struct AsyncEvents {
    pub(crate) workspace_changed_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_added_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_destroyed_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_moved_events: AsyncClosures<MonitorEventData>,
    pub(crate) workspace_rename_events: AsyncClosures<WorkspaceRenameEventData>,
    pub(crate) active_monitor_changed_events: AsyncClosures<MonitorEventData>,
    pub(crate) active_window_changed_events: AsyncClosures<Option<WindowEventData>>,
    pub(crate) fullscreen_state_changed_events: AsyncClosures<bool>,
    pub(crate) monitor_removed_events: AsyncClosures<String>,
    pub(crate) monitor_added_events: AsyncClosures<String>,
    pub(crate) keyboard_layout_change_events: AsyncClosures<LayoutEvent>,
    pub(crate) sub_map_changed_events: AsyncClosures<String>,
    pub(crate) window_open_events: AsyncClosures<WindowOpenEvent>,
    pub(crate) window_close_events: AsyncClosures<Address>,
    pub(crate) window_moved_events: AsyncClosures<WindowMoveEvent>,
    pub(crate) layer_open_events: AsyncClosures<String>,
    pub(crate) layer_closed_events: AsyncClosures<String>,
    pub(crate) float_state_events: AsyncClosures<WindowFloatEventData>,
    pub(crate) urgent_state_events: AsyncClosures<Address>,
    pub(crate) minimize_events: AsyncClosures<MinimizeEventData>,
    pub(crate) window_title_changed_events: AsyncClosures<Address>,
    pub(crate) screencast_events: AsyncClosures<ScreencastEventData>,
}

/// Event data for renameworkspace event
#[derive(Debug, Clone)]
pub struct WorkspaceRenameEventData {
    /// Workspace id
    pub workspace_id: WorkspaceId,
    /// Workspace name content
    pub workspace_name: String,
}

/// Event data for a minimize event
#[derive(Clone, Debug)]
pub struct MinimizeEventData {
    /// Window address
    pub window_address: Address,
    /// whether it's minimized or not
    pub is_minimized: bool,
}

/// Event data for screencast event
#[derive(Debug, Clone, Copy)]
pub struct ScreencastEventData {
    /// State/Is it turning on?
    pub is_turning_on: bool,
    /// Owner type, is it a monitor?
    pub is_monitor: bool,
}

/// The data for the event executed when moving a window to a new workspace
#[derive(Clone, Debug)]
pub struct WindowMoveEvent {
    /// Window address
    pub window_address: Address,
    /// The workspace name
    pub workspace_name: String,
}

#[allow(unsafe_code)]
unsafe impl Send for WindowMoveEvent {}
#[allow(unsafe_code)]
unsafe impl Sync for WindowMoveEvent {}
/// The data for the event executed when opening a new window
#[derive(Clone, Debug)]
pub struct WindowOpenEvent {
    /// Window address
    pub window_address: Address,
    /// The workspace name
    pub workspace_name: String,
    /// Window class
    pub window_class: String,
    /// Window title
    pub window_title: String,
}

#[allow(unsafe_code)]
unsafe impl Send for WindowOpenEvent {}
#[allow(unsafe_code)]
unsafe impl Sync for WindowOpenEvent {}
/// The data for the event executed when changing keyboard layouts
#[derive(Clone, Debug)]
pub struct LayoutEvent {
    /// Keyboard name
    pub keyboard_name: String,
    /// Layout name
    pub layout_name: String,
}

#[allow(unsafe_code)]
unsafe impl Send for LayoutEvent {}
#[allow(unsafe_code)]
unsafe impl Sync for LayoutEvent {}
/// The mutable state available to Closures
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct State {
    /// The active workspace
    pub active_workspace: WorkspaceType,
    /// The active monitor
    pub active_monitor: String,
    /// The fullscreen state
    pub fullscreen_state: bool,
}

#[allow(unsafe_code)]
unsafe impl Send for State {}
#[allow(unsafe_code)]
unsafe impl Sync for State {}
impl State {
    /// Execute changes in state
    pub async fn execute_state(self, old: State) -> crate::Result<Self> {
        let state = self.clone();
        if self != old {
            use crate::dispatch::{Dispatch, DispatchType};
            if old.fullscreen_state != state.fullscreen_state {
                use crate::dispatch::FullscreenType;
                Dispatch::call_async(DispatchType::ToggleFullscreen(FullscreenType::NoParam))
                    .await?;
            }
            if old.active_workspace != state.active_workspace {
                use crate::dispatch::WorkspaceIdentifierWithSpecial;
                Dispatch::call_async(DispatchType::Workspace(match &state.active_workspace {
                    WorkspaceType::Regular(name) => WorkspaceIdentifierWithSpecial::Name(name),
                    WorkspaceType::Special(opt) => {
                        WorkspaceIdentifierWithSpecial::Special(match opt {
                            Some(name) => Some(name),
                            None => None,
                        })
                    }
                }))
                .await?;
            }
            if old.active_monitor != state.active_monitor {
                use crate::dispatch::MonitorIdentifier;
                Dispatch::call_async(DispatchType::FocusMonitor(MonitorIdentifier::Name(
                    &state.active_monitor,
                )))
                .await?;
            };
        }
        Ok(state)
    }
    /// Execute changes in state
    pub fn execute_state_sync(self, old: State) -> crate::Result<Self> {
        let state = self.clone();
        if self != old {
            use crate::dispatch::{Dispatch, DispatchType};
            if old.fullscreen_state != state.fullscreen_state {
                use crate::dispatch::FullscreenType;
                Dispatch::call(DispatchType::ToggleFullscreen(FullscreenType::NoParam))?;
            }
            if old.active_workspace != state.active_workspace {
                use crate::dispatch::WorkspaceIdentifierWithSpecial;
                Dispatch::call(DispatchType::Workspace(match &state.active_workspace {
                    WorkspaceType::Regular(name) => WorkspaceIdentifierWithSpecial::Name(name),
                    WorkspaceType::Special(opt) => {
                        WorkspaceIdentifierWithSpecial::Special(match opt {
                            Some(name) => Some(name),
                            None => None,
                        })
                    }
                }))?;
            }
            if old.active_monitor != state.active_monitor {
                use crate::dispatch::MonitorIdentifier;
                Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Name(
                    &state.active_monitor,
                )))?;
            };
        }
        Ok(state)
    }
}

pub(crate) fn execute_closure<T: Clone>(f: &Closure<T>, val: T) {
    match f {
        EventTypes::MutableState(_) => panic!("Using mutable handler with immutable listener"),
        EventTypes::Regular(fun) => fun(val),
    }
}

pub(crate) async fn execute_closure_async<T>(f: &AsyncClosure<T>, val: T) {
    match f {
        AsyncEventTypes::MutableState(_) => panic!("Using mutable handler with immutable listener"),
        AsyncEventTypes::Regular(fun) => fun(val).await,
    }
}

#[allow(dead_code)]
pub(crate) async fn execute_closure_async_state<T: Clone>(
    f: &AsyncClosure<T>,
    val: T,
    state: &mut State,
) {
    match f {
        AsyncEventTypes::MutableState(fun) => fun(val, state).await,
        AsyncEventTypes::Regular(_) => panic!("Using mutable handler with immutable listener"),
    }
}
pub(crate) async fn execute_closure_mut<T>(
    state: State,
    f: &Closure<T>,
    val: T,
) -> crate::Result<State> {
    let old_state = state.clone();
    let mut new_state = state;
    match f {
        EventTypes::MutableState(fun) => fun(val, &mut new_state),
        EventTypes::Regular(fun) => fun(val),
    }

    let new_state = new_state.execute_state(old_state).await?;
    Ok(new_state)
}

#[allow(clippy::redundant_clone)]
pub(crate) fn execute_closure_mut_sync<T>(
    state: State,
    f: &Closure<T>,
    val: T,
) -> crate::Result<State> {
    let old_state = state.clone();
    let mut new_state = state;
    match f {
        EventTypes::MutableState(fun) => fun(val, &mut new_state),
        EventTypes::Regular(fun) => fun(val),
    }

    let new_state = new_state.execute_state_sync(old_state)?;
    Ok(new_state)
}

/// This tuple struct holds window event data
#[derive(Debug, Clone)]
pub struct WindowEventData {
    /// The window class
    pub window_class: String,
    /// The window title
    pub window_title: String,
    /// The window address
    pub window_address: Address,
}

#[allow(unsafe_code)]
unsafe impl Send for WindowEventData {}
#[allow(unsafe_code)]
unsafe impl Sync for WindowEventData {}
/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct MonitorEventData {
    /// The monitor name
    pub monitor_name: String,
    /// The workspace
    pub workspace: WorkspaceType,
}

#[allow(unsafe_code)]
unsafe impl Send for MonitorEventData {}
#[allow(unsafe_code)]
unsafe impl Sync for MonitorEventData {}
/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct WindowFloatEventData {
    /// The window address
    pub window_address: Address,
    /// The float state
    pub is_floating: bool,
}

#[allow(unsafe_code)]
unsafe impl Send for WindowFloatEventData {}
#[allow(unsafe_code)]
unsafe impl Sync for WindowFloatEventData {}
/// This enum holds every event type
#[derive(Debug, Clone)]
pub(crate) enum Event {
    WorkspaceChanged(WorkspaceType),
    WorkspaceDeleted(WorkspaceType),
    WorkspaceAdded(WorkspaceType),
    WorkspaceMoved(MonitorEventData),
    WorkspaceRename(WorkspaceRenameEventData),
    ActiveWindowChangedV1(Option<(String, String)>),
    ActiveWindowChangedV2(Option<Address>),
    ActiveWindowChangedMerged(Option<WindowEventData>),
    ActiveMonitorChanged(MonitorEventData),
    FullscreenStateChanged(bool),
    MonitorAdded(String),
    MonitorRemoved(String),
    WindowOpened(WindowOpenEvent),
    WindowClosed(Address),
    WindowMoved(WindowMoveEvent),
    LayoutChanged(LayoutEvent),
    SubMapChanged(String),
    LayerOpened(String),
    LayerClosed(String),
    FloatStateChanged(WindowFloatEventData),
    UrgentStateChanged(Address),
    Minimize(MinimizeEventData),
    WindowTitleChanged(Address),
    Screencast(ScreencastEventData),
}

fn check_for_regex_error(val: Result<Regex, RegexError>) -> Regex {
    match val {
        Ok(value) => value,
        Err(RegexError::Syntax(str)) => panic!("syntax error: {str}"),
        Err(RegexError::CompiledTooBig(size)) => {
            panic!("The compiled regex size is too big ({size})")
        }
        Err(_) => unreachable!(),
    }
}

fn parse_string_as_work(str: String) -> WorkspaceType {
    if str == "special" {
        WorkspaceType::Special(None)
    } else if str.starts_with("special:") {
        {
            let mut iter = str.split(':');
            iter.next();
            match iter.next() {
                Some(name) => WorkspaceType::Special(Some(name.to_string())),
                None => WorkspaceType::Special(None),
            }
        }
    } else {
        WorkspaceType::Regular(str)
    }
}

macro_rules! report_unknown {
    ($event:tt) => {
        #[cfg(not(feature = "silent"))]
        eprintln!(
            "A unknown event was passed into Hyprland-rs
            PLEASE MAKE AN ISSUE!!
            The event was: {event}",
            event = $event
        );
    };
}

use std::collections::BTreeSet;

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

use std::sync::Mutex;
static CHECK_TABLE: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

#[derive(PartialEq, Eq, Hash)]
enum ParsedEventType {
    WorkspaceChanged,
    WorkspaceDeleted,
    WorkspaceAdded,
    WorkspaceMoved,
    WorkspaceRename,
    ActiveWindowChangedV1,
    ActiveWindowChangedV2,
    ActiveMonitorChanged,
    FullscreenStateChanged,
    MonitorAdded,
    MonitorRemoved,
    WindowOpened,
    WindowClosed,
    WindowMoved,
    LayoutChanged,
    SubMapChanged,
    LayerOpened,
    LayerClosed,
    FloatStateChanged,
    UrgentStateChanged,
    Minimize,
    WindowTitleChanged,
    Screencast,
    Unknown,
}

/// This internal function parses event strings
pub(crate) fn event_parser(event: String) -> crate::Result<Vec<Event>> {
    static EVENT_SET: Lazy<HashMap<ParsedEventType, Regex>> = Lazy::new(|| {
        vec![
            (
                ParsedEventType::WorkspaceChanged,
                r"\bworkspace>>(?P<workspace>.*)",
            ),
            (
                ParsedEventType::WorkspaceDeleted,
                r"destroyworkspace>>(?P<workspace>.*)",
            ),
            (
                ParsedEventType::WorkspaceAdded,
                r"createworkspace>>(?P<workspace>.*)",
            ),
            (
                ParsedEventType::WorkspaceMoved,
                r"moveworkspace>>(?P<workspace>.*),(?P<monitor>.*)",
            ),
            (
                ParsedEventType::WorkspaceRename,
                r"renameworkspace>>(?P<id>.*),(?P<name>.*)",
            ),
            (
                ParsedEventType::ActiveMonitorChanged,
                r"focusedmon>>(?P<monitor>.*),(?P<workspace>.*)",
            ),
            (
                ParsedEventType::ActiveWindowChangedV1,
                r"activewindow>>(?P<class>.*?),(?P<title>.*)",
            ),
            (
                ParsedEventType::ActiveWindowChangedV2,
                r"activewindowv2>>(?P<address>.*)",
            ),
            (
                ParsedEventType::FullscreenStateChanged,
                r"fullscreen>>(?P<state>0|1)",
            ),
            (
                ParsedEventType::MonitorRemoved,
                r"monitorremoved>>(?P<monitor>.*)",
            ),
            (
                ParsedEventType::MonitorAdded,
                r"monitoradded>>(?P<monitor>.*)",
            ),
            (
                ParsedEventType::WindowOpened,
                r"openwindow>>(?P<address>.*),(?P<workspace>.*),(?P<class>.*),(?P<title>.*)",
            ),
            (
                ParsedEventType::WindowClosed,
                r"closewindow>>(?P<address>.*)",
            ),
            (
                ParsedEventType::WindowMoved,
                r"movewindow>>(?P<address>.*),(?P<workspace>.*)",
            ),
            (
                ParsedEventType::LayoutChanged,
                r"activelayout>>(?P<keyboard>.*)(?P<layout>.*)",
            ),
            (ParsedEventType::SubMapChanged, r"submap>>(?P<submap>.*)"),
            (
                ParsedEventType::LayerOpened,
                r"openlayer>>(?P<namespace>.*)",
            ),
            (
                ParsedEventType::LayerClosed,
                r"closelayer>>(?P<namespace>.*)",
            ),
            (
                ParsedEventType::FloatStateChanged,
                r"changefloatingmode>>(?P<address>.*),(?P<floatstate>[0-1])",
            ),
            (
                ParsedEventType::Minimize,
                r"minimize>>(?P<address>.*),(?P<state>[0-1])",
            ),
            (
                ParsedEventType::Screencast,
                r"screencast>>(?P<state>[0-1]),(?P<owner>[0-1])",
            ),
            (
                ParsedEventType::UrgentStateChanged,
                r"urgent>>(?P<address>.*)",
            ),
            (
                ParsedEventType::WindowTitleChanged,
                r"windowtitle>>(?P<address>.*)",
            ),
            (ParsedEventType::Unknown, r"(?P<Event>^[^>]*)"),
        ]
        .into_iter()
        .map(|(e, r)| (e, check_for_regex_error(Regex::new(r))))
        .collect()
    });

    let event_iter = event.trim().split('\n');

    let mut events: Vec<Event> = vec![];

    for item in event_iter {
        let matched: Vec<_> = EVENT_SET
            .iter()
            .filter(|(_, r)| r.is_match(item))
            .map(|(pet, r)| {
                (
                    pet,
                    r.captures(item).unwrap_or_else(|| {
                        panic!(
                            "Unable to find captures while parsing Hyprland event: {}",
                            item
                        )
                    }),
                )
            })
            .collect();

        let (e, captures) = match matched.len() {
            0 => unreachable!(),
            1 => {
                report_unknown!((item.split('>').next().unwrap_or("unknown")));
                continue;
            }
            2 => matched
                .into_iter()
                .find(|(e, _)| **e != ParsedEventType::Unknown)
                .unwrap_or_else(|| unreachable!()),
            _ => {
                return Err(HyprError::IoError(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Event matched more than one regex (not a unknown event issue!)",
                )));
            }
        };

        match e {
            ParsedEventType::WorkspaceChanged => {
                let captured = &captures["workspace"];
                let workspace = if !captured.is_empty() {
                    parse_string_as_work(captured.to_string())
                } else {
                    WorkspaceType::Regular("1".to_string())
                };
                events.push(Event::WorkspaceChanged(workspace));
            }
            ParsedEventType::WorkspaceDeleted => {
                let workspace = parse_string_as_work(captures["workspace"].to_string());
                events.push(Event::WorkspaceDeleted(workspace));
            }
            ParsedEventType::WorkspaceAdded => {
                let workspace = parse_string_as_work(captures["workspace"].to_string());
                events.push(Event::WorkspaceAdded(workspace));
            }
            ParsedEventType::WorkspaceMoved => {
                let workspace = parse_string_as_work(captures["workspace"].to_string());
                let monitor = &captures["monitor"];
                events.push(Event::WorkspaceMoved(MonitorEventData {
                    monitor_name: monitor.to_string(),
                    workspace,
                }));
            }
            ParsedEventType::WorkspaceRename => {
                let id = &captures["id"];
                let name = &captures["name"];
                events.push(Event::WorkspaceRename(WorkspaceRenameEventData {
                    workspace_id: id
                        .parse::<WorkspaceId>()
                        .map_err(|e| HyprError::IoError(std::io::Error::other(e)))?,
                    workspace_name: name.to_string(),
                }));
            }
            ParsedEventType::ActiveMonitorChanged => {
                let monitor = &captures["monitor"];
                let workspace = &captures["workspace"];
                events.push(Event::ActiveMonitorChanged(MonitorEventData {
                    monitor_name: monitor.to_string(),
                    workspace: WorkspaceType::Regular(workspace.to_string()),
                }));
            }
            ParsedEventType::ActiveWindowChangedV1 => {
                let class = &captures["class"];
                let title = &captures["title"];
                if !class.is_empty() && !title.is_empty() {
                    events.push(Event::ActiveWindowChangedV1(Some((
                        class.to_string(),
                        title.to_string(),
                    ))));
                } else {
                    events.push(Event::ActiveWindowChangedV1(None));
                }
            }
            ParsedEventType::ActiveWindowChangedV2 => {
                let addr = &captures["address"];
                if addr != "," {
                    events.push(Event::ActiveWindowChangedV2(Some(Address::new(
                        format_event_addr(addr),
                    ))));
                } else {
                    events.push(Event::ActiveWindowChangedV2(None));
                }
            }
            ParsedEventType::FullscreenStateChanged => {
                let state = &captures["state"] != "0";
                events.push(Event::FullscreenStateChanged(state))
            }
            ParsedEventType::MonitorRemoved => {
                let monitor = &captures["monitor"];
                events.push(Event::MonitorRemoved(monitor.to_string()));
            }
            ParsedEventType::MonitorAdded => {
                let monitor = &captures["monitor"];
                events.push(Event::MonitorAdded(monitor.to_string()));
            }
            ParsedEventType::WindowOpened => {
                let addr = format_event_addr(&captures["address"]);
                let workspace = &captures["workspace"];
                let class = &captures["class"];
                let title = &captures["title"];
                events.push(Event::WindowOpened(WindowOpenEvent {
                    window_address: Address::new(addr),
                    workspace_name: workspace.to_string(),
                    window_class: class.to_string(),
                    window_title: title.to_string(),
                }));
            }
            ParsedEventType::WindowClosed => {
                let addr = format_event_addr(&captures["address"]);
                events.push(Event::WindowClosed(Address::new(addr)));
            }
            ParsedEventType::WindowMoved => {
                let addr = format_event_addr(&captures["address"]);
                let work = &captures["workspace"];
                events.push(Event::WindowMoved(WindowMoveEvent {
                    window_address: Address::new(addr),
                    workspace_name: work.to_string(),
                }));
            }
            ParsedEventType::LayoutChanged => {
                let keyboard_name = &captures["keyboard"];
                let layout = &captures["layout"];
                events.push(Event::LayoutChanged(LayoutEvent {
                    keyboard_name: keyboard_name.to_string(),
                    layout_name: layout.to_string(),
                }));
            }
            ParsedEventType::SubMapChanged => {
                let submap = &captures["submap"];
                events.push(Event::SubMapChanged(submap.to_string()));
            }
            ParsedEventType::LayerOpened => {
                let namespace = &captures["namespace"];
                events.push(Event::LayerOpened(namespace.to_string()));
            }
            ParsedEventType::LayerClosed => {
                let namespace = &captures["namespace"];
                events.push(Event::LayerClosed(namespace.to_string()));
            }
            ParsedEventType::FloatStateChanged => {
                let addr = format_event_addr(&captures["address"]);
                let state = &captures["floatstate"] == "0"; // FIXME: does 0 mean it's floating?
                events.push(Event::FloatStateChanged(WindowFloatEventData {
                    window_address: Address::new(addr),
                    is_floating: state,
                }));
            }
            ParsedEventType::Minimize => {
                let addr = format_event_addr(&captures["address"]);
                let state = &captures["state"] == "1";
                events.push(Event::Minimize(MinimizeEventData {
                    window_address: Address::new(addr),
                    is_minimized: state,
                }));
            }
            ParsedEventType::Screencast => {
                let state = &captures["state"] == "1";
                let owner = &captures["owner"] == "1";
                events.push(Event::Screencast(ScreencastEventData {
                    is_turning_on: state,
                    is_monitor: owner,
                }));
            }
            ParsedEventType::UrgentStateChanged => {
                let addr = format_event_addr(&captures["address"]);
                events.push(Event::UrgentStateChanged(Address::new(addr)));
            }
            ParsedEventType::WindowTitleChanged => {
                let addr = format_event_addr(&captures["address"]);
                events.push(Event::WindowTitleChanged(Address::new(addr)));
            }
            ParsedEventType::Unknown => {
                #[cfg(not(feature = "silent"))]
                match &captures.name("event") {
                    Some(s) => {
                        let table = CHECK_TABLE.lock();
                        if let Ok(mut tbl) = table {
                            let should_run = tbl.insert(s.as_str().to_string());
                            if should_run {
                                eprintln!(
                                    "A unknown event was passed into Hyprland-rs
                        PLEASE MAKE AN ISSUE!!
                        The event was: {}",
                                    s.as_str()
                                );
                            }
                        }
                    }
                    None => {
                        let table = CHECK_TABLE.lock();
                        if let Ok(mut tbl) = table {
                            let should_run = tbl.insert("unknown".to_string());
                            if should_run {
                                eprintln!(
                            "A unknown event was passed into Hyprland-rs\nPLEASE MAKE AN ISSUE!!\nThe event was: {item}"
                        );
                            }
                        }
                    }
                };
            }
        }
    }

    Ok(events)
}

fn format_event_addr(addr: &str) -> String {
    format!("0x{addr}")
}
