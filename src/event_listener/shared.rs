use crate::shared::WorkspaceId;
use regex::{Error as RegexError, Regex, RegexSet};
use std::io;

pub(crate) enum EventTypes<T: ?Sized, U: ?Sized> {
    MutableState(Box<U>),
    Regular(Box<T>),
}

pub(crate) type Closure<T> = EventTypes<dyn Fn(T), dyn Fn(T, &mut State)>;
pub(crate) type Closures<T> = Vec<Closure<T>>;

#[allow(clippy::type_complexity)]
pub(crate) struct Events {
    pub(crate) workspace_changed_events: Closures<WorkspaceId>,
    pub(crate) workspace_added_events: Closures<WorkspaceId>,
    pub(crate) workspace_destroyed_events: Closures<WorkspaceId>,
    pub(crate) active_monitor_changed_events: Closures<MonitorEventData>,
    pub(crate) active_window_changed_events: Closures<Option<WindowEventData>>,
    pub(crate) fullscreen_state_changed_events: Closures<bool>,
    pub(crate) monitor_removed_events: Closures<String>,
    pub(crate) monitor_added_events: Closures<String>,
}

/// The mutable state available to Closures
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct State {
    /// The active workspace
    pub active_workspace: WorkspaceId,
    /// The active monitor
    pub active_monitor: String,
    /// The fullscreen state
    pub fullscreen_state: bool,
}

impl State {
    /// Execute changes in state
    pub async fn execute_state(self, old: State) -> io::Result<Self> {
        let state = self.clone();
        if self != old {
            use crate::dispatch::{dispatch, DispatchType};
            if old.fullscreen_state != state.fullscreen_state {
                use crate::dispatch::FullscreenType;
                dispatch(DispatchType::ToggleFullscreen(FullscreenType::NoParam)).await?;
            }
            if old.active_workspace != state.active_workspace {
                use crate::dispatch::WorkspaceIdentifierWithSpecial;
                dispatch(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
                    state.active_workspace,
                )))
                .await?;
            }
            if old.active_monitor != state.active_monitor {
                use crate::dispatch::MonitorIdentifier;
                dispatch(DispatchType::FocusMonitor(MonitorIdentifier::Name(
                    state.active_monitor.clone(),
                )))
                .await?;
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

pub(crate) async fn execute_closure_mut<T>(
    state: State,
    f: &Closure<T>,
    val: T,
) -> io::Result<State> {
    let old_state = state.clone();
    let mut new_state = state.clone();
    match f {
        EventTypes::MutableState(fun) => fun(val, &mut new_state),
        EventTypes::Regular(fun) => fun(val),
    }

    let new_state = new_state.execute_state(old_state).await?;
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
    pub WorkspaceId,
);

/// This enum holds every event type
#[derive(Debug, Clone)]
pub(crate) enum Event {
    WorkspaceChanged(WorkspaceId),
    WorkspaceDeleted(WorkspaceId),
    WorkspaceAdded(WorkspaceId),
    ActiveWindowChanged(Option<WindowEventData>),
    ActiveMonitorChanged(MonitorEventData),
    FullscreenStateChanged(bool),
    MonitorAdded(String),
    MonitorRemoved(String),
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

/// This internal function parses event strings
pub(crate) fn event_parser(event: String) -> io::Result<Vec<Event>> {
    lazy_static! {
        static ref EVENT_SET: RegexSet = check_for_regex_set_error(RegexSet::new(&[
            r"\bworkspace>>(?P<workspace>[0-9]{1,2}|)",
            r"destroyworkspace>>(?P<workspace>[0-9]{1,2})",
            r"createworkspace>>(?P<workspace>[0-9]{1,2})",
            r"focusedmon>>(?P<monitor>.*),(?P<workspace>[0-9]{1,2})",
            r"activewindow>>(?P<class>.*),(?P<title>.*)",
            r"fullscreen>>(?P<state>0|1)",
            r"monitorremoved>>(?P<monitor>.*)",
            r"monitoradded>>(?P<monitor>.*)"
        ]));
        static ref EVENT_REGEXES: Vec<Regex> = EVENT_SET
            .patterns()
            .iter()
            .map(|pat| check_for_regex_error(Regex::new(pat)))
            .collect();
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
            panic!("something has went down -{:#?}-", matches_event)
        };

        if matches_event.len() == 1 {
            match matches_event[0] {
                0 => {
                    // WorkspaceChanged
                    let captured = &captures["workspace"];
                    let workspace = if !captured.is_empty() {
                        match captured.parse::<u8>() {
                            Ok(num) => num,
                            Err(e) => panic!("error parsing string as u8: {e}"),
                        }
                    } else {
                        1_u8
                    };
                    events.push(Event::WorkspaceChanged(workspace));
                }
                1 => {
                    // destroyworkspace
                    let workspace = match captures["workspace"].parse::<u8>() {
                        Ok(num) => num,
                        Err(e) => panic!("error parsing string as u8: {e}"),
                    };
                    events.push(Event::WorkspaceDeleted(workspace));
                }
                2 => {
                    // WorkspaceAdded
                    let workspace = match captures["workspace"].parse::<u8>() {
                        Ok(num) => num,
                        Err(e) => panic!("error parsing string as u8: {e}"),
                    };
                    events.push(Event::WorkspaceAdded(workspace));
                }
                3 => {
                    // ActiveMonitorChanged
                    let monitor = &captures["monitor"];
                    let workspace = match captures["workspace"].parse::<u8>() {
                        Ok(num) => num,
                        Err(e) => panic!("error parsing string as u8: {e}"),
                    };
                    events.push(Event::ActiveMonitorChanged(MonitorEventData(
                        monitor.to_string(),
                        workspace,
                    )));
                }
                4 => {
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
                5 => {
                    // FullscreenStateChanged
                    let state = &captures["state"] != "0";
                    events.push(Event::FullscreenStateChanged(state))
                }
                6 => {
                    // MonitorRemoved
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorRemoved(monitor.to_string()));
                }
                7 => {
                    // MonitorAdded
                    let monitor = &captures["monitor"];
                    events.push(Event::MonitorAdded(monitor.to_string()));
                }
                _ => panic!("How did this happen?"),
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown event"));
        }
    }

    Ok(events)
}
