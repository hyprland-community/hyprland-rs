use crate::shared::*;
use once_cell::sync::Lazy;
use regex::{Error as RegexError, Regex};
use std::{fmt::Debug, pin::Pin};

#[cfg(test)]
mod test;

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

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub(crate) type EventType<T> = Box<T>;
pub(crate) type AsyncEventType<T> = Pin<Box<T>>;

pub(crate) type VoidFuture = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;

pub(crate) type Closure<T> = EventType<dyn Fn(T)>;
pub(crate) type AsyncClosure<T> = AsyncEventType<dyn Sync + Send + Fn(T) -> VoidFuture>;
pub(crate) type Closures<T> = Vec<Closure<T>>;
pub(crate) type AsyncClosures<T> = Vec<AsyncClosure<T>>;

pub(crate) struct Events {
    pub(crate) workspace_changed_events: Closures<WorkspaceType>,
    pub(crate) workspace_added_events: Closures<WorkspaceType>,
    pub(crate) workspace_added_v2_events: Closures<WorkspaceV2Data>,
    pub(crate) workspace_destroyed_events: Closures<WorkspaceV2Data>,
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
    pub(crate) workspace_added_v2_events: AsyncClosures<WorkspaceV2Data>,
    pub(crate) workspace_destroyed_events: AsyncClosures<WorkspaceV2Data>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRenameEventData {
    /// Workspace id
    pub workspace_id: WorkspaceId,
    /// Workspace name content
    pub workspace_name: String,
}

/// Event data for a minimize event
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinimizeEventData {
    /// Window address
    pub window_address: Address,
    /// whether it's minimized or not
    pub is_minimized: bool,
}

/// Event data for screencast event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreencastEventData {
    /// State/Is it turning on?
    pub is_turning_on: bool,
    /// Owner type, is it a monitor?
    pub is_monitor: bool,
}

/// The data for the event executed when moving a window to a new workspace
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowMoveEvent {
    /// Window address
    pub window_address: Address,
    /// The workspace name
    pub workspace_name: String,
}

/// The data for the event executed when opening a new window
#[derive(Clone, Debug, PartialEq)]
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

/// The data for the event executed when changing keyboard layouts
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutEvent {
    /// Keyboard name
    pub keyboard_name: String,
    /// Layout name
    pub layout_name: String,
}

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
    f(val);
}

pub(crate) async fn execute_closure_async<T>(f: &AsyncClosure<T>, val: T) {
    f(val).await;
}

/// This tuple struct holds window event data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowEventData {
    /// The window class
    pub window_class: String,
    /// The window title
    pub window_title: String,
    /// The window address
    pub window_address: Address,
}

/// This tuple struct holds monitor event data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorEventData {
    /// The monitor name
    pub monitor_name: String,
    /// The workspace
    pub workspace: WorkspaceType,
}

/// This tuple struct holds monitor event data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowFloatEventData {
    /// The window address
    pub window_address: Address,
    /// The float state
    pub is_floating: bool,
}

/// This tuple struct workspacev2 creation and deletion data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceV2Data {
    /// Workspace Id
    pub workspace_id: WorkspaceId,
    /// Workspace name
    pub workspace_name: WorkspaceType,
}

/// This enum holds every event type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Event {
    WorkspaceChanged(WorkspaceType),
    WorkspaceDeleted(WorkspaceV2Data),
    WorkspaceAdded(WorkspaceType),
    WorkspaceAddedV2(WorkspaceV2Data),
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
    ($event:expr) => {
        #[cfg(not(feature = "silent"))]
        eprintln!(
            "An unknown event was passed into Hyprland-rs
            PLEASE MAKE AN ISSUE!!
            The event was: {event}",
            event = $event
        );
    };
}

#[cfg(feature = "ahash")]
use ahash::{HashSet, HashSetExt};
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

#[cfg(feature = "parking_lot")]
use parking_lot::Mutex;
#[cfg(not(feature = "parking_lot"))]
use std::sync::Mutex;

static CHECK_TABLE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum ParsedEventType {
    WorkspaceChanged,
    WorkspaceDeletedV2,
    WorkspaceAdded,
    WorkspaceAddedV2,
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

/// All the recognized events
static EVENT_SET: Lazy<Box<[(ParsedEventType, Regex)]>> = Lazy::new(|| {
    [
        (
            ParsedEventType::WorkspaceChanged,
            r"\bworkspace>>(?P<workspace>.*)",
        ),
        (
            ParsedEventType::WorkspaceDeletedV2,
            r"destroyworkspacev2>>(?P<id>.*),(?P<name>.*)",
        ),
        (
            ParsedEventType::WorkspaceAdded,
            r"createworkspace>>(?P<workspace>.*)",
        ),
        (
            ParsedEventType::WorkspaceAddedV2,
            r"createworkspacev2>>(?P<id>.*),(?P<name>.*)",
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
    ].into_iter()
    .map(|(e, r)| (
        e,
        match Regex::new(r) {
            Ok(value) => value,
            Err(e) => {
                // I believe that panics here are fine because the chances of the library user finding them are extremely high
                // This check does occur at runtime though...
                eprintln!("An internal error occured in hyprland-rs while parsing regex! Please open an issue!");
                match e {
                    RegexError::Syntax(str) => panic!("Regex syntax error: {str}"),
                    RegexError::CompiledTooBig(size) => {
                        panic!("The compiled regex size is too big! ({size})")
                    }
                    _ => panic!("Error compiling regex: {e}"),
                }
            }
        })
    ).collect()
});

/// This internal function parses event strings
pub(crate) fn event_parser(event: String) -> crate::Result<Vec<Event>> {
    // TODO: Optimize nested looped regex capturing. Maybe pull in rayon if possible.
    let event_iter = event
        .trim()
        .lines()
        .map(|event_line| {
            let type_matches = EVENT_SET
                .iter()
                .filter_map(|(event_type, regex)| Some((event_type, regex.captures(event_line)?)))
                .collect::<Vec<_>>();

            (event_line, type_matches)
        })
        .filter(|(_, b)| !b.is_empty());

    let mut temp_event_holder = Vec::new();

    for (event_str, matches) in event_iter {
        match matches.len() {
            0 => hypr_err!(
                "A Hyprland event that has no regex matches was passed! Please file a bug report!"
            ),
            1 => {
                report_unknown!((event_str.split('>').next().unwrap_or("unknown")));
                continue;
            }
            2 => {
                let (event_type, captures) = match matches
                .into_iter()
                .find(|(e, _)| **e != ParsedEventType::Unknown) {
                    Some(t) => t,
                    None => hypr_err!("The only events captured were unknown Hyprland events! Please file a bug report!"),
                };

                temp_event_holder.push((event_str, event_type, captures));
            }
            _ => {
                hypr_err!("Event matched more than one regex (not an unknown event issue!)");
            }
        }
    }

    let parsed_events = temp_event_holder
        .into_iter()
        .map(|(event_str, event_type, captures)| match event_type {
            ParsedEventType::WorkspaceChanged => {
                let captured = &captures["workspace"];
                let workspace = if !captured.is_empty() {
                    parse_string_as_work(captured.to_string())
                } else {
                    WorkspaceType::Regular("1".to_string())
                };
                Ok(Event::WorkspaceChanged(workspace))
            }
            ParsedEventType::WorkspaceDeletedV2 => Ok(Event::WorkspaceDeleted(WorkspaceV2Data {
                workspace_id: captures["id"].parse::<WorkspaceId>().map_err(|e| HyprError::Internal(format!("Workspace delete v2: invalid integer error: {e}")))?,
                workspace_name: parse_string_as_work(captures["name"].to_string())
            })),
            ParsedEventType::WorkspaceAdded => Ok(Event::WorkspaceAdded(parse_string_as_work(
                captures["workspace"].to_string(),
            ))),
            ParsedEventType::WorkspaceAddedV2 => Ok(Event::WorkspaceAddedV2(
                WorkspaceV2Data{
                    workspace_id: captures["id"].parse::<WorkspaceId>().map_err(|e| HyprError::Internal(format!("Workspace delete v2: invalid integer error: {e}")))?,
                    workspace_name: parse_string_as_work(captures["name"].to_string()),
                }
            )),
           ParsedEventType::WorkspaceMoved => Ok(Event::WorkspaceMoved(MonitorEventData {
                monitor_name: captures["monitor"].to_string(),
                workspace: parse_string_as_work(captures["workspace"].to_string()),
            })),
            ParsedEventType::WorkspaceRename => {
                Ok(Event::WorkspaceRename(WorkspaceRenameEventData {
                    workspace_id: captures["id"]
                        .parse::<WorkspaceId>()
                        .map_err(|e| HyprError::Internal(format!("Workspace rename: invalid integer error: {e}")))?,
                    workspace_name: captures["name"].to_string(),
                }))
            }
            ParsedEventType::ActiveMonitorChanged => {
                Ok(Event::ActiveMonitorChanged(MonitorEventData {
                    monitor_name: captures["monitor"].to_string(),
                    workspace: WorkspaceType::Regular(captures["workspace"].to_string()),
                }))
            }
            ParsedEventType::ActiveWindowChangedV1 => {
                let class = &captures["class"];
                let title = &captures["title"];
                let event = if !class.is_empty() && !title.is_empty() {
                    Event::ActiveWindowChangedV1(Some((class.to_string(), title.to_string())))
                } else {
                    Event::ActiveWindowChangedV1(None)
                };

                Ok(event)
            }
            ParsedEventType::ActiveWindowChangedV2 => {
                let addr = &captures["address"];
                let event = if addr != "," {
                    Event::ActiveWindowChangedV2(Some(Address::fmt_new(addr)))
                } else {
                    Event::ActiveWindowChangedV2(None)
                };
                Ok(event)
            }
            ParsedEventType::FullscreenStateChanged => {
                let state = &captures["state"] != "0";
                Ok(Event::FullscreenStateChanged(state))
            }
            ParsedEventType::MonitorRemoved => {
                Ok(Event::MonitorRemoved(captures["monitor"].to_string()))
            }
            ParsedEventType::MonitorAdded => {
                Ok(Event::MonitorAdded(captures["monitor"].to_string()))
            }
            ParsedEventType::WindowOpened => Ok(Event::WindowOpened(WindowOpenEvent {
                window_address: Address::fmt_new(&captures["address"]),
                workspace_name: captures["workspace"].to_string(),
                window_class: captures["class"].to_string(),
                window_title: captures["title"].to_string(),
            })),
            ParsedEventType::WindowClosed => Ok(Event::WindowClosed(Address::fmt_new(&captures["address"]))),
            ParsedEventType::WindowMoved => Ok(Event::WindowMoved(WindowMoveEvent {
                window_address: Address::fmt_new(&captures["address"]),
                workspace_name: captures["workspace"].to_string(),
            })),
            ParsedEventType::LayoutChanged => Ok(Event::LayoutChanged(LayoutEvent {
                keyboard_name: captures["keyboard"].to_string(),
                layout_name: captures["layout"].to_string(),
            })),
            ParsedEventType::SubMapChanged => {
                Ok(Event::SubMapChanged(captures["submap"].to_string()))
            }
            ParsedEventType::LayerOpened => {
                Ok(Event::LayerOpened(captures["namespace"].to_string()))
            }
            ParsedEventType::LayerClosed => {
                Ok(Event::LayerClosed(captures["namespace"].to_string()))
            }
            ParsedEventType::FloatStateChanged => {
                let state = &captures["floatstate"] == "0"; // FIXME: does 0 mean it's floating?
                Ok(Event::FloatStateChanged(WindowFloatEventData {
                    window_address: Address::fmt_new(&captures["address"]),
                    is_floating: state,
                }))
            }
            ParsedEventType::Minimize => {
                let state = &captures["state"] == "1";
                Ok(Event::Minimize(MinimizeEventData {
                    window_address: Address::fmt_new(&captures["address"]),
                    is_minimized: state,
                }))
            }
            ParsedEventType::Screencast => {
                let state = &captures["state"] == "1";
                let owner = &captures["owner"] == "1";
                Ok(Event::Screencast(ScreencastEventData {
                    is_turning_on: state,
                    is_monitor: owner,
                }))
            }
            ParsedEventType::UrgentStateChanged => Ok(Event::UrgentStateChanged(Address::fmt_new(&captures["address"]))),
            ParsedEventType::WindowTitleChanged => Ok(Event::WindowTitleChanged(Address::fmt_new(&captures["address"]))),
            ParsedEventType::Unknown => {
                #[cfg(not(feature = "silent"))]
                {
                    let table = CHECK_TABLE.lock();
                    // The std mutex returns a Result, the parking_lot mutex does not. This is a hack that allows us to
                    // keep the table code how it is, without duplicating or `return`ing.
                    #[cfg(feature = "parking_lot")]
                    let table = Ok::<_, std::convert::Infallible>(table);

                    if let Ok(mut tbl) = table {
                        let (event_string, print_str) =
                            match captures.name("event").map(|s| s.as_str()) {
                                Some(s) => (s.to_string(), s),
                                None => ("Unknown".to_owned(), event_str),
                            };

                        let should_run = tbl.insert(event_string);
                        if should_run {
                            eprintln!(
                                "An unknown event was passed into Hyprland-rs\nPLEASE MAKE AN ISSUE!!\nThe event was: {print_str}"
                            );
                        }
                    }
                }
                hypr_err!("Unknown event: {event_str}");
            }
        });

    let mut events: Vec<Event> = Vec::new();

    for event in parsed_events {
        events.push(event?);
    }

    // if events.is_empty() {
    //     hypr_err!("No events!");
    // }

    Ok(events)
}
