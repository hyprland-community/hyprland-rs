//! # Dispatch module
//!
//! This module is used for calling dispatchers and changing keywords
//!
//! ## Usage
//!
//! ```rust
//! use hyprland::Result;
//! use hyprland::dispatch::{Dispatch, DispatchType};
//! fn main() -> Result<()> {
//!     Dispatch::call(DispatchType::Exec("kitty"))?;
//!
//!    Ok(())
//! }
//! ````

use crate::default_instance;
use crate::dispatch::fmt::*;
use crate::error::HyprError;
use crate::shared::*;
use derive_more::Display;
use std::string::ToString;

/// This enum is for identifying a window
#[derive(Debug, Clone, Display)]
pub enum WindowIdentifier<'a> {
    /// The address of a window
    #[display("address:{_0}")]
    Address(Address),
    /// A Regular Expression to match the window class (handled by Hyprland)
    #[display("class:{_0}")]
    ClassRegularExpression(&'a str),
    /// The window title
    #[display("title:{_0}")]
    Title(&'a str),
    /// The window's process Id
    #[display("pid:{_0}")]
    ProcessId(u32),
}

/// This enum holds the fullscreen types
#[derive(Debug, Clone, Display)]
pub enum FullscreenType {
    /// Fills the whole screen
    #[display("0")]
    Real,
    /// Maximizes the window
    #[display("1")]
    Maximize,
    /// Passes no param
    #[display("")]
    NoParam,
}

/// This enum holds directions, typically used for moving
#[derive(Debug, Clone, Display)]
#[allow(missing_docs)]
pub enum Direction {
    #[display("u")]
    Up,
    #[display("d")]
    Down,
    #[display("r")]
    Right,
    #[display("l")]
    Left,
}

/// This enum is used for resizing and moving windows precisely
#[derive(Debug, Clone)]
pub enum Position {
    /// A delta
    Delta(i16, i16),
    /// The exact size
    Exact(i16, i16),
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Position::Delta(x, y) => format!("{x} {y}"),
            Position::Exact(w, h) => format!("exact {w} {h}"),
        };
        write!(f, "{out}")
    }
}

/// This enum holds a direction for cycling
#[allow(missing_docs)]
#[derive(Debug, Clone, Display)]
pub enum CycleDirection {
    #[display("")]
    Next,
    #[display("prev")]
    Previous,
}

/// This enum holds a direction for switch windows in a group
#[allow(missing_docs)]
#[derive(Debug, Clone, Display)]
pub enum WindowSwitchDirection {
    #[display("b")]
    Back,
    #[display("f")]
    Forward,
}

/// This enum is used for identifying monitors
#[derive(Debug, Clone)]
pub enum MonitorIdentifier<'a> {
    /// The monitor that is to the specified direction of the active one
    Direction(Direction),
    /// The monitor id
    Id(MonitorId),
    /// The monitor name
    Name(&'a str),
    /// The current monitor
    Current,
    /// The workspace relative to the current workspace
    Relative(i32),
}

impl std::fmt::Display for MonitorIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            MonitorIdentifier::Direction(dir) => dir.to_string(),
            MonitorIdentifier::Id(id) => id.to_string(),
            MonitorIdentifier::Name(name) => name.to_string(),
            MonitorIdentifier::Current => "current".to_string(),
            MonitorIdentifier::Relative(int) => format_relative(*int, ""),
        };
        write!(f, "{out}")
    }
}

/// This enum holds corners
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Corner {
    BottomLeft = 0,
    BottomRight = 1,
    TopRight = 2,
    TopLeft = 3,
}

/// This enum holds options that are applied to the current workspace
#[derive(Debug, Clone, Display)]
pub enum WorkspaceOptions {
    /// Makes all windows pseudo tiled
    #[display("allfloat")]
    AllPseudo,
    /// Makes all windows float
    #[display("allpseudo")]
    AllFloat,
}

/// This struct holds options for the first empty workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstEmpty {
    /// If the first empty workspace should be on the monitor
    pub on_monitor: bool,
    /// If the first empty workspace should be next
    pub next: bool,
}

impl std::fmt::Display for FirstEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.on_monitor {
            s.push('m');
        }
        if self.next {
            s.push('n');
        }
        write!(f, "{s}")
    }
}

/// This enum is for identifying workspaces that also includes the special workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum WorkspaceIdentifierWithSpecial<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    #[display("{}", format_relative(*_0, ""))]
    Relative(i32),
    /// The workspace on the monitor relative to the current workspace
    #[display("{}", format_relative(*_0, "m"))]
    RelativeMonitor(i32),
    /// The workspace on the monitor relative to the current workspace, including empty workspaces
    #[display("{}", format_relative(*_0, "r"))]
    RelativeMonitorIncludingEmpty(i32),
    /// The open workspace relative to the current workspace
    #[display("{}", format_relative(*_0, "e"))]
    RelativeOpen(i32),
    /// The previous Workspace
    #[display("previous")]
    Previous,
    /// The previous Workspace
    #[display("previous_per_monitor")]
    PreviousPerMonitor,
    /// The first available empty workspace
    #[display("{}", format!("empty{}", _0))]
    Empty(FirstEmpty),
    /// The name of the workspace
    #[display("name:{_0}")]
    Name(&'a str),
    /// The special workspace
    #[display("special{}", format_special_workspace_ident(_0))]
    Special(Option<&'a str>),
}

pub(super) mod fmt {
    #[inline(always)]
    pub(super) fn format_special_workspace_ident<'a>(opt: &'a Option<&'a str>) -> String {
        match opt {
            Some(o) => ":".to_owned() + o,
            None => String::new(),
        }
    }

    #[inline(always)]
    pub(super) fn format_relative(int: i32, extra: &'_ str) -> String {
        if int.is_positive() {
            format!("{extra}+{int}")
        } else if int.is_negative() {
            format!("{extra}-{}", int.abs())
        } else {
            "+0".to_owned()
        }
    }
}

/// This enum is for identifying workspaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceIdentifier<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    Relative(i32),
    /// The workspace on the monitor relative to the current workspace
    RelativeMonitor(i32),
    /// The workspace on the monitor relative to the current workspace, including empty workspaces
    RelativeMonitorIncludingEmpty(i32),
    /// The open workspace relative to the current workspace
    RelativeOpen(i32),
    /// The previous Workspace
    Previous,
    /// The first available empty workspace
    Empty,
    /// The name of the workspace
    Name(&'a str),
}

impl std::fmt::Display for WorkspaceIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WorkspaceIdentifier::*;
        let out = match self {
            Id(id) => format!("{id}"),
            Name(name) => format!("name:{name}"),
            Relative(int) => format_relative(*int, ""),
            RelativeMonitor(int) => format_relative(*int, "m"),
            RelativeMonitorIncludingEmpty(int) => format_relative(*int, "r"),
            RelativeOpen(int) => format_relative(*int, "e"),
            Previous => "previous".to_string(),
            Empty => "empty".to_string(),
        };

        write!(f, "{out}")
    }
}

/// This enum is the params to [DispatchType::MoveWindow] dispatcher
#[derive(Debug, Clone)]
pub enum WindowMove<'a> {
    /// Moves the window to a specified monitor
    Monitor(MonitorIdentifier<'a>),
    /// Moves the window in a specified direction
    Direction(Direction),
}

/// This enum holds the signals
#[derive(Debug, Clone, Copy)]
pub enum SignalType {
    /// Hangup detected on controlling terminal
    SIGHUP = 1,
    /// Interrupt from keyboard
    SIGINT = 2,
    /// Quit from keyboard
    SIGQUIT = 3,
    /// Illegal Instruction
    SIGILL = 4,
    /// Trace/breakpoint trap
    SIGTRAP = 5,
    /// Abort signal from abort
    SIGABRT = 6,
    /// Bus error (bad memory access)
    SIGBUS = 7,
    /// Erroneous arithmetic operation
    SIGFPE = 8,
    /// Kill signal
    SIGKILL = 9,
    /// User-defined signal 1
    SIGUSR1 = 10,
    /// Invalid memory reference
    SIGSEGV = 11,
    /// User-defined signal 2
    SIGUSR2 = 12,
    /// Broken pipe
    SIGPIPE = 13,
    /// Timer signal from alarm
    SIGALRM = 14,
    /// Termination signal
    SIGTERM = 15,
    /// Stack fault on coprocessor
    SIGSTKFLT = 16,
    /// Child stopped, terminated, or continued
    SIGCHLD = 17,
    /// Continue if stopped
    SIGCONT = 18,
    /// Stop process
    SIGSTOP = 19,
    /// Stop typed at terminal
    SIGTSTP = 20,
    /// Terminal input for background process
    SIGTTIN = 21,
    /// Terminal output for background process
    SIGTTOU = 22,
    /// Urgent condition on socket
    SIGURG = 23,
    /// CPU time limit exceeded
    SIGXCPU = 24,
    /// File size limit exceeded
    SIGXFSZ = 25,
    /// Virtual alarm clock
    SIGVTALRM = 26,
    /// Profiling timer expired
    SIGPROF = 27,
    /// Window resize signal
    SIGWINCH = 28,
    /// I/O now possible
    SIGIO = 29,
    /// Power failure
    SIGPWR = 30,
    /// Bad system call
    SIGSYS = 31,
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

/// This enum holds every dispatcher
#[derive(Debug, Clone)]
pub enum DispatchType<'a> {
    /// This lets you use dispatchers not supported by hyprland-rs yet, please make issues before
    /// using
    Custom(
        /// Name of event
        &'a str,
        /// Args
        &'a str,
    ),
    /// This dispatcher changes the current cursor
    SetCursor(
        /// The cursor theme
        &'a str,
        /// The size
        u16,
    ),
    /// This dispatcher executes a program
    Exec(&'a str),
    /// This dispatcher passes a keybind to a window when called in a
    /// keybind, its used for global keybinds. And should **ONLY** be used with keybinds
    Pass(WindowIdentifier<'a>),
    /// Executes a Global Shortcut using the GlobalShortcuts portal.
    Global(&'a str),
    /// This dispatcher kills the active window/client
    KillActiveWindow,
    /// This dispatcher closes the specified window
    CloseWindow(WindowIdentifier<'a>),
    /// This dispatcher changes the current workspace
    Workspace(WorkspaceIdentifierWithSpecial<'a>),
    /// This dispatcher moves a window (focused if not specified) to a workspace
    MoveToWorkspace(
        WorkspaceIdentifierWithSpecial<'a>,
        Option<WindowIdentifier<'a>>,
    ),
    /// This dispatcher moves a window (focused if not specified) to a workspace, without switching to that
    /// workspace
    MoveToWorkspaceSilent(
        WorkspaceIdentifierWithSpecial<'a>,
        Option<WindowIdentifier<'a>>,
    ),
    /// This dispatcher floats a window (current if not specified)
    ToggleFloating(Option<WindowIdentifier<'a>>),
    /// This dispatcher toggles the current window fullscreen state
    ToggleFullscreen(FullscreenType),
    /// This dispatcher toggles the focused window’s internal
    /// fullscreen state without altering the geometry
    ToggleFakeFullscreen,
    /// This dispatcher sets the DPMS status for all monitors
    ToggleDPMS(bool, Option<&'a str>),
    /// This dispatcher toggles pseudo tiling for the current window
    TogglePseudo,
    /// This dispatcher pins the active window to all workspaces
    TogglePin,
    /// This dispatcher pins the specified window to all workspaces
    TogglePinWindow(WindowIdentifier<'a>),
    /// This dispatcher sends a signal to the active window
    Signal(SignalType),
    /// This dispatcher sends a signal to the specified window
    SignalWindow(WindowIdentifier<'a>, SignalType),
    /// This dispatcher moves the window focus in a specified direction
    MoveFocus(Direction),
    /// This dispatcher moves the current window to a monitor or in a specified direction
    MoveWindow(WindowMove<'a>),
    /// This dispatcher centers the active window
    CenterWindow,
    /// This dispatcher resizes the active window using a [Position] enum
    ResizeActive(Position),
    /// This dispatcher moves the active window using a [Position] enum
    MoveActive(Position),
    /// This dispatcher resizes the specified window using a [Position] enum
    ResizeWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher moves the specified window using a [Position] enum
    MoveWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher cycles windows using a specified direction
    CycleWindow(CycleDirection),
    /// This dispatcher swaps the focused window with the window on a workspace using a specified direction
    SwapNext(CycleDirection),
    /// This dispatcher swaps windows using a specified direction
    SwapWindow(Direction),
    /// This dispatcher focuses a specified window
    FocusWindow(WindowIdentifier<'a>),
    /// This dispatcher focuses a specified monitor
    FocusMonitor(MonitorIdentifier<'a>),
    /// This dispatcher changed the split ratio
    ChangeSplitRatio(f32),
    /// This dispatcher toggle opacity for the current window/client
    ToggleOpaque,
    /// This dispatcher moves the cursor to a specified corner of a window
    MoveCursorToCorner(Corner),
    /// This dispatcher moves the cursor to a specified position
    /// (x, y) where x starts from left to right, and y starts from top to bottom
    MoveCursor(i64, i64),
    /// This dispatcher applied a option to all windows in a workspace
    WorkspaceOption(WorkspaceOptions),
    /// This dispatcher renames a workspace
    RenameWorkspace(WorkspaceId, Option<&'a str>),
    /// This exits Hyprland **(DANGEROUS)**
    Exit,
    /// This dispatcher forces the renderer to reload
    ForceRendererReload,
    /// This dispatcher moves the current workspace to a specified monitor
    MoveCurrentWorkspaceToMonitor(MonitorIdentifier<'a>),
    /// This dispatcher moves a specified workspace to a specified monitor
    MoveWorkspaceToMonitor(WorkspaceIdentifier<'a>, MonitorIdentifier<'a>),
    /// This dispatcher swaps the active workspaces of two monitors
    SwapActiveWorkspaces(MonitorIdentifier<'a>, MonitorIdentifier<'a>),
    /// This dispatcher brings the active window to the top of the stack
    BringActiveToTop,
    /// This toggles the special workspace (AKA scratchpad)
    ToggleSpecialWorkspace(Option<String>),
    /// This dispatcher jump to urgent or the last window
    FocusUrgentOrLast,
    /// Switch focus from current to previously focused window
    FocusCurrentOrLast,

    // LAYOUT DISPATCHERS
    // DWINDLE
    /// Toggles the split (top/side) of the current window. `preserve_split` must be enabled for toggling to work.
    ToggleSplit,

    // MASTER
    /// Swaps the current window with master.
    /// If the current window is the master,
    /// swaps it with the first child.
    SwapWithMaster(SwapWithMasterParam),
    /// Focuses the master window.
    FocusMaster(FocusMasterParam),
    /// Adds a master to the master side. That will be the active window,
    /// if it’s not a master, or the first non-master window.
    AddMaster,
    /// Removes a master from the master side. That will be the
    /// active window, if it’s a master, or the last master window.
    RemoveMaster,
    /// Sets the orientation for the current workspace to left
    /// (master area left, slave windows to the right, vertically stacked)
    OrientationLeft,
    /// Sets the orientation for the current workspace to right
    /// (master area right, slave windows to the left, vertically stacked)
    OrientationRight,
    /// Sets the orientation for the current workspace to top
    /// (master area top, slave windows to the bottom, horizontally stacked)
    OrientationTop,
    /// Sets the orientation for the current workspace to bottom
    /// (master area bottom, slave windows to the top, horizontally stacked)
    OrientationBottom,
    /// Sets the orientation for the current workspace to center
    /// (master area center, slave windows alternate to the left and right, vertically stacked)
    OrientationCenter,
    /// Cycle to the next orientation for the current workspace (clockwise)
    OrientationNext,
    /// Cycle to the previous orientation for the current workspace (counter-clockwise)
    OrientationPrev,

    // Group Dispatchers
    /// Toggles the current active window into a group
    ToggleGroup,
    /// Switches to the next window in a group.
    ChangeGroupActive(WindowSwitchDirection),
    /// Locks the groups
    LockGroups(LockType),
    /// Moves the active window into a group in a specified direction
    MoveIntoGroup(Direction),
    /// Moves the active window out of a group.
    MoveOutOfGroup,
}

/// Enum used with [DispatchType::LockGroups], to determine how to lock/unlock
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum LockType {
    /// Lock Group
    #[display("lock")]
    Lock,
    /// Unlock Group
    #[display("unlock")]
    Unlock,
    /// Toggle lock state of Group
    #[display("toggle")]
    ToggleLock,
}

/// Param for [DispatchType::SwapWithMaster] dispatcher
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum SwapWithMasterParam {
    /// New focus is the new master window
    #[display("master")]
    Master,
    /// New focus is the new child
    #[display("child")]
    Child,
    /// Keep the focus of the previously focused window
    #[display("auto")]
    Auto,
}

/// Param for [DispatchType::FocusMaster] dispatcher
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum FocusMasterParam {
    /// Focus stays at master, (even if it was selected before)
    #[display("master")]
    Master,
    /// If the current window is the master, focuses the first child
    #[display("auto")]
    Auto,
}

pub(crate) fn gen_dispatch_str(cmd: DispatchType, dispatch: bool) -> crate::Result<CommandContent> {
    use DispatchType::*;
    let sep = if dispatch { " " } else { "," };
    let string_to_pass = match &cmd {
        Custom(name, args) => format!("{name}{sep}{args}"),
        Exec(sh) => format!("exec{sep}{sh}"),
        Pass(win) => format!("pass{sep}{win}"),
        Global(name) => format!("global{sep}{name}"),
        KillActiveWindow => "killactive".to_string(),
        CloseWindow(win) => format!("closewindow{sep}{win}"),
        Workspace(work) => format!("workspace{sep}{work}"),
        MoveToWorkspace(work, Some(win)) => format!("movetoworkspace{sep}{work},{win}"),
        MoveToWorkspace(work, None) => format!("movetoworkspace{sep}{work}"),
        MoveToWorkspaceSilent(work, Some(win)) => format!("movetoworkspacesilent{sep}{work},{win}"),
        MoveToWorkspaceSilent(work, None) => format!("movetoworkspacesilent{sep}{work}"),
        ToggleFloating(Some(v)) => format!("togglefloating{sep}{v}"),
        ToggleFloating(None) => "togglefloating".to_string(),
        ToggleFullscreen(ftype) => format!("fullscreen{sep}{ftype}"),
        ToggleFakeFullscreen => "fakefullscreen".to_string(),
        ToggleDPMS(stat, mon) => {
            format!(
                "dpms{sep}{} {}",
                if *stat { "on" } else { "off" },
                mon.unwrap_or_default()
            )
        }
        TogglePseudo => "pseudo".to_string(),
        TogglePin => "pin".to_string(),
        TogglePinWindow(win) => format!("pin{sep}{win}"),
        MoveFocus(dir) => format!("movefocus{sep}{dir}",),
        MoveWindow(ident) => format!(
            "movewindow{sep}{}",
            match ident {
                WindowMove::Direction(dir) => dir.to_string(),
                WindowMove::Monitor(mon) => format!("mon:{mon}"),
            }
        ),
        CenterWindow => "centerwindow".to_string(),
        ResizeActive(pos) => format!("resizeactive{sep}{pos}"),
        MoveActive(pos) => format!("moveactive {pos}"),
        ResizeWindowPixel(pos, win) => format!("resizewindowpixel{sep}{pos},{win}"),
        MoveWindowPixel(pos, win) => format!("movewindowpixel{sep}{pos},{win}"),
        CycleWindow(dir) => format!("cyclenext{sep}{dir}"),
        SwapNext(dir) => format!("swapnext{sep}{dir}"),
        SwapWindow(dir) => format!("swapwindow{sep}{dir}"),
        FocusWindow(win) => format!("focuswindow{sep}{win}"),
        FocusMonitor(mon) => format!("focusmonitor{sep}{mon}"),
        ChangeSplitRatio(ratio) => format!("splitratio {ratio}"),
        ToggleOpaque => "toggleopaque".to_string(),
        MoveCursorToCorner(corner) => format!("movecursortocorner{sep}{}", corner.clone() as u8),
        MoveCursor(x, y) => format!("movecursor{sep}{x} {y}"),
        WorkspaceOption(opt) => format!("workspaceopt{sep}{opt}"),
        Exit => "exit".to_string(),
        ForceRendererReload => "forcerendererreload".to_string(),
        MoveCurrentWorkspaceToMonitor(mon) => format!("movecurrentworkspacetomonitor{sep}{mon}"),
        MoveWorkspaceToMonitor(work, mon) => format!("moveworkspacetomonitor{sep}{work} {mon}"),
        ToggleSpecialWorkspace(Some(name)) => format!("togglespecialworkspace {name}"),
        ToggleSpecialWorkspace(None) => "togglespecialworkspace".to_string(),
        RenameWorkspace(id, name) => {
            format!(
                "renameworkspace{sep}{id} {}",
                name.unwrap_or(&id.to_string())
            )
        }
        SwapActiveWorkspaces(mon, mon2) => format!("swapactiveworkspaces{sep}{mon} {mon2}",),
        BringActiveToTop => "bringactivetotop".to_string(),
        SetCursor(theme, size) => format!("{theme} {}", *size),
        Signal(sig) => format!("signal{sep}{sig}"),
        SignalWindow(win, sig) => format!("signalwindow{sep}{win},{sig}"),
        FocusUrgentOrLast => "focusurgentorlast".to_string(),
        FocusCurrentOrLast => "focuscurrentorlast".to_string(),
        ToggleSplit => "togglesplit".to_string(),
        SwapWithMaster(param) => format!("swapwithmaster{sep}{param}"),
        FocusMaster(param) => format!("focusmaster{sep}{param}"),
        AddMaster => "addmaster".to_string(),
        RemoveMaster => "removemaster".to_string(),
        OrientationLeft => "orientationleft".to_string(),
        OrientationRight => "orientationright".to_string(),
        OrientationTop => "orientationtop".to_string(),
        OrientationBottom => "orientationbottom".to_string(),
        OrientationCenter => "orientationcenter".to_string(),
        OrientationNext => "orientationnext".to_string(),
        OrientationPrev => "orientationprev".to_string(),
        ToggleGroup => "togglegroup".to_string(),
        ChangeGroupActive(dir) => format!("changegroupactive{sep}{dir}"),
        LockGroups(how) => format!("lockgroups{sep}{how}"),
        MoveIntoGroup(dir) => format!("moveintogroup{sep}{dir}"),
        MoveOutOfGroup => "moveoutofgroup".to_string(),
    };

    if let SetCursor(_, _) = cmd {
        Ok(command!(JSON, "setcursor {string_to_pass}"))
    } else if dispatch {
        Ok(command!(JSON, "dispatch {string_to_pass}"))
    } else {
        Ok(command!(Empty, "{string_to_pass}"))
    }
}

/// The struct that provides all dispatching methods
pub struct Dispatch;

impl Dispatch {
    /// This function calls a specified dispatcher (blocking)
    ///
    /// ```rust
    /// # use hyprland::Result;
    /// # fn main() -> Result<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call(DispatchType::Exec("kitty"))
    /// # }
    /// ```
    pub fn call(dispatch_type: DispatchType) -> crate::Result<()> {
        Self::instance_call(default_instance()?, dispatch_type)
    }

    /// This function calls a specified dispatcher (blocking)
    ///
    /// ```rust
    /// # use hyprland::Result;
    /// # fn main() -> Result<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// let instance = hyprland::instance::Instance::from_current_env()?;
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::instance_call(&instance, DispatchType::Exec("kitty"))
    /// # }
    /// ```
    pub fn instance_call(
        instance: &crate::instance::Instance,
        dispatch_type: DispatchType,
    ) -> crate::Result<()> {
        let output = instance.write_to_socket(gen_dispatch_str(dispatch_type, true)?);
        match output {
            Ok(msg) => match msg.as_str() {
                "ok" => Ok(()),
                msg => Err(HyprError::NotOkDispatch(msg.to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// This function calls a specified dispatcher (async)
    ///
    /// ```rust
    /// # use hyprland::Result;
    /// # async fn main() -> Result<()> {
    /// use hyprland::dispatch::{Dispatch,DispatchType};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call_async(DispatchType::Exec("kitty")).await
    /// # }
    /// ```
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn call_async(dispatch_type: DispatchType<'_>) -> crate::Result<()> {
        Self::instance_call_async(default_instance()?, dispatch_type).await
    }

    /// This function calls a specified dispatcher (async)
    ///
    /// ```rust
    /// # use hyprland::Result;
    /// # async fn main() -> Result<()> {
    /// use hyprland::dispatch::{Dispatch,DispatchType};
    /// let instance = hyprland::instance::Instance::from_current_env()?;
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::instance_call_async(&instance, DispatchType::Exec("kitty")).await
    /// # }
    /// ```
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn instance_call_async(
        instance: &crate::instance::Instance,
        dispatch_type: DispatchType<'_>,
    ) -> crate::Result<()> {
        let output = instance
            .write_to_socket_async(gen_dispatch_str(dispatch_type, true)?)
            .await;
        match output {
            Ok(msg) => match msg.as_str() {
                "ok" => Ok(()),
                msg => Err(HyprError::NotOkDispatch(msg.to_string())),
            },
            Err(error) => Err(error),
        }
    }
}

/// Macro abstraction over [Dispatch::call]
#[macro_export]
macro_rules! dispatch {
    (async; $instance:expr, $dis:ident, $( $arg:expr ), *) => {
        $crate::dispatch::Dispatch::instance_call_async($instance, $crate::dispatch::DispatchType::$dis($($arg), *))
    };
    (async; $dis:ident, $( $arg:expr ), *) => {
        $crate::dispatch::Dispatch::call_async($crate::dispatch::DispatchType::$dis($($arg), *))
    };
    ($instance:expr, $dis:ident, $( $arg:expr ), *) => {
        $crate::dispatch::Dispatch::instance_call($instance, $crate::dispatch::DispatchType::$dis($($arg), *))
    };
    ($dis:ident, $( $arg:expr ), *) => {
        $crate::dispatch::Dispatch::call($crate::dispatch::DispatchType::$dis($($arg), *))
    };
}
