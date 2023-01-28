use crate::shared::*;
use regex::{Error as RegexError, Regex, RegexSet};
use std::io;

/// This trait provides shared behaviour for listener types
#[async_trait]
pub trait Listener {
    /// This method starts the event listener
    fn start_listener() -> HResult<()>;

    /// This method starts the event listener (async)
    async fn start_listener_async() -> HResult<()>;
}

pub(crate) enum EventTypes<T: ?Sized, U: ?Sized> {
    MutableState(Box<U>),
    Regular(Box<T>),
}

pub(crate) type Closure<T> = EventTypes<dyn Fn(T), dyn Fn(T, &mut State)>;
pub(crate) type Closures<T> = Vec<Closure<T>>;

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
}

/// The data for the event executed when moving a window to a new workspace
#[derive(Clone, Debug)]
pub struct WindowMoveEvent(
    /// Window address
    pub Address,
    /// The workspace name
    pub String,
);

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

/// The data for the event executed when changing keyboard layouts
#[derive(Clone, Debug)]
pub struct LayoutEvent(
    /// Keyboard name
    pub String,
    /// Layout name
    pub String,
);

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

impl State {
    /// Execute changes in state
    pub async fn execute_state(self, old: State) -> HResult<Self> {
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
    pub fn execute_state_sync(self, old: State) -> HResult<Self> {
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

pub(crate) fn execute_closure<T>(f: &Closure<T>, val: T) {
    match f {
        EventTypes::MutableState(_) => panic!("Using mutable handler with immutable listener"),
        EventTypes::Regular(fun) => fun(val),
    }
}

pub(crate) async fn execute_closure_mut<T>(state: State, f: &Closure<T>, val: T) -> HResult<State> {
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
pub(crate) fn execute_closure_mut_sync<T>(state: State, f: &Closure<T>, val: T) -> HResult<State> {
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
);

/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct MonitorEventData(
    /// The monitor name
    pub String,
    /// The workspace
    pub WorkspaceType,
);

/// This tuple struct holds monitor event data
#[derive(Debug, Clone)]
pub struct WindowFloatEventData(
    /// The window address
    pub Address,
    /// The float state
    pub bool,
);

/// This enum holds every event type
#[derive(Debug, Clone)]
pub(crate) enum Event {
    WorkspaceChanged(WorkspaceType),
    WorkspaceDeleted(WorkspaceType),
    WorkspaceAdded(WorkspaceType),
    WorkspaceMoved(MonitorEventData),
    ActiveWindowChanged(Option<WindowEventData>),
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
        eprintln!(
            "A unknown event was passed into Hyprland-rs
            PLEASE MAKE AN ISSUE!!
            The event was: {event}",
            event = $event
        );
    };
}

/// This internal function parses event strings
pub(crate) fn event_parser(event: String) -> HResult<Vec<Event>> {
    lazy_static! {
        static ref EVENT_SET: RegexSet = check_for_regex_set_error(RegexSet::new([
            r"\bworkspace>>(?P<workspace>.*)",
            r"destroyworkspace>>(?P<workspace>.*)",
            r"createworkspace>>(?P<workspace>.*)",
            r"moveworkspace>>(?P<workspace>.*),(?P<monitor>.*)",
            r"focusedmon>>(?P<monitor>.*),(?P<workspace>.*)",
            r"activewindow>>(?P<class>.*),(?P<title>.*)",
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
                        events.push(Event::ActiveWindowChanged(Some(WindowEventData(
                            class.to_string(),
                            title.to_string(),
                        ))));
                    } else {
                        events.push(Event::ActiveWindowChanged(None));
                    }
                }
                6 => {
                    // FullscreenStateChanged
                    let state = &captures["state"] != "0";
                    events.push(Event::FullscreenStateChanged(state))
                }
                7 => {
                    // MonitorRemoved
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorRemoved(monitor.to_string()));
                }
                8 => {
                    // MonitorAdded
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorAdded(monitor.to_string()));
                }
                9 => {
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
                10 => {
                    // WindowClosed
                    let addr = &captures["address"];
                    events.push(Event::WindowClosed(Address::new(addr)));
                }
                11 => {
                    // WindowMoved
                    let addr = &captures["address"];
                    let work = &captures["workspace"];
                    events.push(Event::WindowMoved(WindowMoveEvent(
                        Address::new(addr),
                        work.to_string(),
                    )));
                }
                12 => {
                    // LayoutChanged
                    let keeb = &captures["keyboard"];
                    let layout = &captures["layout"];
                    events.push(Event::LayoutChanged(LayoutEvent(
                        keeb.to_string(),
                        layout.to_string(),
                    )));
                }
                13 => {
                    // SubMapChanged
                    let submap = &captures["submap"];
                    events.push(Event::SubMapChanged(submap.to_string()));
                }
                14 => {
                    // OpenLayer
                    let namespace = &captures["namespace"];
                    events.push(Event::LayerOpened(namespace.to_string()));
                }
                15 => {
                    // CloseLayer
                    let namespace = &captures["namespace"];
                    events.push(Event::LayerClosed(namespace.to_string()));
                }
                16 => {
                    // FloatStateChanged
                    let addr = &captures["address"];
                    let state = &captures["floatstate"] == "0";
                    events.push(Event::FloatStateChanged(WindowFloatEventData(
                        Address::new(addr),
                        state,
                    )));
                }
                18 => {
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
                match &captures.name("event") {
                    Some(s) => eprintln!(
                        "A unknown event was passed into Hyprland-rs
                    PLEASE MAKE AN ISSUE!!
                    The event was: {}",
                        s.as_str()
                    ),
                    None => eprintln!(
                        "A unknown event was passed into Hyprland-rs\nPLEASE MAKE AN ISSUE!!\nThe event was: (Unable to get)"
                    ),
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
