use crate::shared::*;
use regex::{Error as RegexError, Regex, RegexSet};
use std::fmt::Debug;
use std::io;
use std::pin::Pin;

/// This trait provides shared behaviour for listener types
#[async_trait]
pub trait Listener {
    /// This method starts the event listener
    fn start_listener() -> crate::Result<()>;

    /// This method starts the event listener (async)
    async fn start_listener_async() -> crate::Result<()>;
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

pub(crate) type VoidFuture = std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send>>;
pub(crate) type VoidFutureMut =
    std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>>;

pub(crate) type Closure<T> = EventTypes<dyn Fn(T), dyn Fn(T, &mut State)>;
pub(crate) type AsyncClosure<T> = AsyncEventTypes<
    dyn Sync + Send + Fn(T) -> VoidFuture,
    dyn Sync + Send + Fn(T, &mut StateV2) -> VoidFutureMut,
>;
pub(crate) type Closures<T> = Vec<Closure<T>>;
pub(crate) type AsyncClosures<T> = Vec<AsyncClosure<T>>;

#[allow(clippy::type_complexity)]
pub(crate) struct Events {
    pub(crate) workspace_changed_events: Closures<WorkspaceType>,
    pub(crate) workspace_added_events: Closures<WorkspaceType>,
    pub(crate) workspace_destroyed_events: Closures<WorkspaceType>,
    pub(crate) workspace_moved_events: Closures<MonitorEventData>,
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
    pub(crate) screencopy_events: Closures<ScreencopyEventData>,
}

#[allow(clippy::type_complexity)]
pub(crate) struct AsyncEvents {
    pub(crate) workspace_changed_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_added_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_destroyed_events: AsyncClosures<WorkspaceType>,
    pub(crate) workspace_moved_events: AsyncClosures<MonitorEventData>,
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
    pub(crate) screencopy_events: AsyncClosures<ScreencopyEventData>,
}

/// Event data for a minimize event
#[derive(Clone, Debug)]
pub struct MinimizeEventData(
    /// Window address
    pub Address,
    /// Minimize state
    pub bool,
);

/// Event data for screencopy event
#[derive(Debug, Clone, Copy)]
pub struct ScreencopyEventData(
    /// State/Is it turning on?
    pub bool,
    /// Owner type, is it a monitor?
    pub bool,
);

/// The data for the event executed when moving a window to a new workspace
#[derive(Clone, Debug)]
pub struct WindowMoveEvent(
    /// Window address
    pub Address,
    /// The workspace name
    pub String,
);

unsafe impl Send for WindowMoveEvent {}
unsafe impl Sync for WindowMoveEvent {}
/// The data for the event executed when opening a new window
#[derive(Clone, Debug)]
pub struct WindowOpenEvent(
    /// Window address
    pub Address,
    /// The workspace name
    pub String,
    /// Window class
    pub String,
    /// Window title
    pub String,
);

unsafe impl Send for WindowOpenEvent {}
unsafe impl Sync for WindowOpenEvent {}
/// The data for the event executed when changing keyboard layouts
#[derive(Clone, Debug)]
pub struct LayoutEvent(
    /// Keyboard name
    pub String,
    /// Layout name
    pub String,
);

unsafe impl Send for LayoutEvent {}
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

use std::ops::{Deref, DerefMut};
/// Wrapper type that adds handler for events
#[derive(Clone)]
#[doc(hidden)]
pub struct MutWrapper<'a, 'b, T>(T, &'a (dyn Fn(T) + 'a), &'b (dyn Fn(T) -> VoidFuture + 'b));
impl<T> MutWrapper<'_, '_, T> {
    #[allow(dead_code)]
    pub(crate) fn update(&mut self, v: T) {
        self.0 = v;
    }
}
impl<T: PartialEq> PartialEq for MutWrapper<'_, '_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Debug> Debug for MutWrapper<'_, '_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: Eq> Eq for MutWrapper<'_, '_, T> {}

impl<T> Deref for MutWrapper<'_, '_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone> DerefMut for MutWrapper<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1(self.0.clone());
        &mut self.0
    }
}

/// The mutable state available to Closures
#[derive(PartialEq, Eq, Clone, Debug)]
#[doc(hidden)]
pub struct StateV2 {
    /// The active workspace
    pub workspace: MutWrapper<'static, 'static, String>,
    /// The active monitor
    pub monitor: MutWrapper<'static, 'static, String>,
    /// The fullscreen state
    pub fullscreen: MutWrapper<'static, 'static, bool>,
}

unsafe impl Send for StateV2 {}
unsafe impl Sync for StateV2 {}
impl StateV2 {
    /// Init new state
    pub fn new<Str: ToString>(work: Str, mon: Str, full: bool) -> Self {
        use crate::dispatch::*;
        use hyprland_macros::async_closure;
        Self {
            workspace: MutWrapper(
                work.to_string(),
                &|work| {
                    if let Ok(()) =
                        crate::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Name(&work))
                    {
                    }
                },
                &async_closure! { |work| if let Ok(()) =
                crate::dispatch!(async; Workspace, WorkspaceIdentifierWithSpecial::Name(&work)).await {}},
            ),
            monitor: MutWrapper(mon.to_string(), &|_| {}, &async_closure! {|_| {}}),
            fullscreen: MutWrapper(full, &|_| {}, &async_closure! {|_| {}}),
        }
    }
}

unsafe impl Send for State {}
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
        Ok(state.clone())
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

pub(crate) fn execute_closure<T: Clone>(f: &Closure<T>, val: &T) {
    match f {
        EventTypes::MutableState(_) => panic!("Using mutable handler with immutable listener"),
        EventTypes::Regular(fun) => fun(val.clone()),
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
    val: &T,
    state: &mut StateV2,
) {
    match f {
        AsyncEventTypes::MutableState(fun) => fun(val.clone(), state).await,
        AsyncEventTypes::Regular(_) => panic!("Using mutable handler with immutable listener"),
    }
}
pub(crate) async fn execute_closure_mut<T>(
    state: State,
    f: &Closure<T>,
    val: T,
) -> crate::Result<State> {
    let old_state = state.clone();
    let mut new_state = state.clone();
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
    let mut new_state = state.clone();
    match f {
        EventTypes::MutableState(fun) => fun(val, &mut new_state),
        EventTypes::Regular(fun) => fun(val),
    }

    let new_state = new_state.execute_state_sync(old_state)?;
    Ok(new_state)
}

/// This tuple struct holds window event data
#[derive(Debug, Clone)]
pub struct WindowEventData(
    /// The window class
    pub String,
    /// The window title
    pub String,
    /// The window address
    pub Address,
);

unsafe impl Send for WindowEventData {}
unsafe impl Sync for WindowEventData {}
/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct MonitorEventData(
    /// The monitor name
    pub String,
    /// The workspace
    pub WorkspaceType,
);

unsafe impl Send for MonitorEventData {}
unsafe impl Sync for MonitorEventData {}
/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct WindowFloatEventData(
    /// The window address
    pub Address,
    /// The float state
    pub bool,
);

unsafe impl Send for WindowFloatEventData {}
unsafe impl Sync for WindowFloatEventData {}
/// This enum holds every event type
#[derive(Debug, Clone)]
pub(crate) enum Event {
    WorkspaceChanged(WorkspaceType),
    WorkspaceDeleted(WorkspaceType),
    WorkspaceAdded(WorkspaceType),
    WorkspaceMoved(MonitorEventData),
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
    Screencopy(ScreencopyEventData),
}

fn check_for_regex_error(val: Result<Regex, RegexError>) -> Regex {
    match val {
        Ok(value) => value,
        Err(RegexError::Syntax(str)) => panic!("syntax error: {str}"),
        Err(RegexError::CompiledTooBig(size)) => {
            panic!("The compiled regex size is too big ({size})")
        }
        Err(RegexError::__Nonexhaustive) => unreachable!(),
    }
}

fn check_for_regex_set_error(val: Result<RegexSet, RegexError>) -> RegexSet {
    match val {
        Ok(value) => value,
        Err(RegexError::Syntax(str)) => panic!("syntax error: {str}"),
        Err(RegexError::CompiledTooBig(size)) => {
            panic!("The compiled regex size is too big ({size})")
        }
        Err(RegexError::__Nonexhaustive) => unreachable!(),
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
use std::sync::Mutex;
static CHECK_TABLE: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

/// This internal function parses event strings
pub(crate) fn event_parser(event: String) -> crate::Result<Vec<Event>> {
    lazy_static! {
        static ref EVENT_SET: RegexSet = check_for_regex_set_error(RegexSet::new([
            r"\bworkspace>>(?P<workspace>.*)",
            r"destroyworkspace>>(?P<workspace>.*)",
            r"createworkspace>>(?P<workspace>.*)",
            r"moveworkspace>>(?P<workspace>.*),(?P<monitor>.*)",
            r"focusedmon>>(?P<monitor>.*),(?P<workspace>.*)",
            r"activewindow>>(?P<class>.*),(?P<title>.*)",
            r"activewindowv2>>(?P<address>.*)",
            r"fullscreen>>(?P<state>0|1)",
            r"monitorremoved>>(?P<monitor>.*)",
            r"monitoradded>>(?P<monitor>.*)",
            r"openwindow>>(?P<address>.*),(?P<workspace>.*),(?P<class>.*),(?P<title>.*)",
            r"closewindow>>(?P<address>.*)",
            r"movewindow>>(?P<address>.*),(?P<workspace>.*)",
            r"activelayout>>(?P<keyboard>.*)(?P<layout>.*)",
            r"submap>>(?P<submap>.*)",
            r"openlayer>>(?P<namespace>.*)",
            r"closelayer>>(?P<namespace>.*)",
            r"changefloatingmode>>(?P<address>.*),(?P<floatstate>[0-1])",
            r"minimize>>(?P<address>.*),(?P<state>[0-1])",
            r"screencopy>>(?P<state>[0-1]),(?P<owner>[0-1])",
            r"urgent>>(?P<address>.*)",
            r"(?P<Event>.*)>>.*?"
        ]));
        static ref EVENT_REGEXES: Vec<Regex> = EVENT_SET
            .patterns()
            .iter()
            .map(|pat| check_for_regex_error(Regex::new(pat)))
            .collect();
        static ref EVENT_LEN: usize = EVENT_SET.len() - 1;
    }

    let event_iter = event.trim().split('\n');

    let mut events: Vec<Event> = vec![];

    for item in event_iter {
        let matches = EVENT_SET.matches(item);
        let matches_event: Vec<_> = matches.into_iter().collect();
        let captures = if !EVENT_REGEXES.is_empty() && !matches_event.is_empty() {
            match EVENT_REGEXES[matches_event[0]].captures(item) {
                Some(captures) => captures,
                None => panic!("Regex has no captures"),
            }
        } else {
            report_unknown!((item.split('>').collect::<Vec<&str>>()[0]));
            continue;
        };

        if matches_event.len() == 2 {
            match matches_event[0] {
                0 => {
                    // WorkspaceChanged
                    let captured = &captures["workspace"];
                    let workspace = if !captured.is_empty() {
                        parse_string_as_work(captured.to_string())
                    } else {
                        WorkspaceType::Regular("1".to_string())
                    };
                    events.push(Event::WorkspaceChanged(workspace));
                }
                1 => {
                    // destroyworkspace
                    let workspace = parse_string_as_work(captures["workspace"].to_string());
                    events.push(Event::WorkspaceDeleted(workspace));
                }
                2 => {
                    // WorkspaceAdded
                    let workspace = parse_string_as_work(captures["workspace"].to_string());
                    events.push(Event::WorkspaceAdded(workspace));
                }
                3 => {
                    // WorkspaceMoved
                    let workspace = parse_string_as_work(captures["workspace"].to_string());
                    let monitor = &captures["monitor"];
                    events.push(Event::WorkspaceMoved(MonitorEventData(
                        monitor.to_string(),
                        workspace,
                    )));
                }
                4 => {
                    // ActiveMonitorChanged
                    let monitor = &captures["monitor"];
                    let workspace = &captures["workspace"];
                    events.push(Event::ActiveMonitorChanged(MonitorEventData(
                        monitor.to_string(),
                        WorkspaceType::Regular(workspace.to_string()),
                    )));
                }
                5 => {
                    // ActiveWindowChanged
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
                6 => {
                    // ActiveWindowChangedV2
                    let addr = &captures["address"];
                    if addr != "," {
                        events.push(Event::ActiveWindowChangedV2(Some(Address::new(addr))));
                    } else {
                        events.push(Event::ActiveWindowChangedV2(None));
                    }
                }
                7 => {
                    // FullscreenStateChanged
                    let state = &captures["state"] != "0";
                    events.push(Event::FullscreenStateChanged(state))
                }
                8 => {
                    // MonitorRemoved
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorRemoved(monitor.to_string()));
                }
                9 => {
                    // MonitorAdded
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorAdded(monitor.to_string()));
                }
                10 => {
                    // WindowOpened
                    let addr = &captures["address"];
                    let workspace = &captures["workspace"];
                    let class = &captures["class"];
                    let title = &captures["title"];
                    events.push(Event::WindowOpened(WindowOpenEvent(
                        Address::new(addr),
                        workspace.to_string(),
                        class.to_string(),
                        title.to_string(),
                    )));
                }
                11 => {
                    // WindowClosed
                    let addr = &captures["address"];
                    events.push(Event::WindowClosed(Address::new(addr)));
                }
                12 => {
                    // WindowMoved
                    let addr = &captures["address"];
                    let work = &captures["workspace"];
                    events.push(Event::WindowMoved(WindowMoveEvent(
                        Address::new(addr),
                        work.to_string(),
                    )));
                }
                13 => {
                    // LayoutChanged
                    let keeb = &captures["keyboard"];
                    let layout = &captures["layout"];
                    events.push(Event::LayoutChanged(LayoutEvent(
                        keeb.to_string(),
                        layout.to_string(),
                    )));
                }
                14 => {
                    // SubMapChanged
                    let submap = &captures["submap"];
                    events.push(Event::SubMapChanged(submap.to_string()));
                }
                15 => {
                    // OpenLayer
                    let namespace = &captures["namespace"];
                    events.push(Event::LayerOpened(namespace.to_string()));
                }
                16 => {
                    // CloseLayer
                    let namespace = &captures["namespace"];
                    events.push(Event::LayerClosed(namespace.to_string()));
                }
                17 => {
                    // FloatStateChanged
                    let addr = &captures["address"];
                    let state = &captures["floatstate"] == "0";
                    events.push(Event::FloatStateChanged(WindowFloatEventData(
                        Address::new(addr),
                        state,
                    )));
                }
                18 => {
                    // ScreenCopyStateChanged
                    let state = &captures["state"] == "1";
                    let owner = &captures["owner"] == "1";
                    events.push(Event::Screencopy(ScreencopyEventData(state, owner)));
                }
                19 => {
                    // MinimizeStateChanged
                    let addr = &captures["address"];
                    let state = &captures["state"] == "1";
                    events.push(Event::Minimize(MinimizeEventData(
                        Address::new(addr),
                        state,
                    )));
                }
                20 => {
                    // UrgentStateChanged
                    let addr = &captures["address"];
                    events.push(Event::UrgentStateChanged(Address::new(addr)));
                }
                _ => unreachable!(), //panic!("There are only 16 items in the array? prob a regex issue 🤷"),
            }
        } else if matches_event.len() == 1 {
            if matches_event[0] != *EVENT_LEN {
                panic!("One event matched, that isn't the unrecognised event type, sus")
            } else {
                // Unknown Event
                #[cfg(not(feature = "silent"))]
                match &captures.name("event") {
                    Some(s) => {
                        let table = CHECK_TABLE.lock();
                        if let Ok(mut tbl) = table {
                            let should_run = tbl.insert(s.as_str().to_string());
                            if should_run == true {
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
                            if should_run == true {
                                eprintln!(
                        "A unknown event was passed into Hyprland-rs\nPLEASE MAKE AN ISSUE!!\nThe event was: {item}"
                    );
                            }
                        }
                    }
                };
            }
        } else {
            return Err(HyprError::IoError(io::Error::new(
                io::ErrorKind::InvalidData,
                "Event matched more or less than one regex (not a unknown event issue!)",
            )));
        }
    }

    Ok(events)
}
