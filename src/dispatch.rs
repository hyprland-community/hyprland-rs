//! # Dispatch module
//!
//! This module is used for calling dispatchers and changing keywords
//!
//! ## Usage
//!
//! ```rust
//! use hyprland::shared::HResult;
//! use hyprland::dispatch::{Dispatch, DispatchType};
//! fn main() -> HResult<()> {
//!    Dispatch::call(DispatchType::Exec("kitty"))?;
//!
//!    Ok(())
//! }
//! ````

use crate::shared::*;

/// This enum is for identifying a window
#[derive(Debug, Clone)]
pub enum WindowIdentifier<'a> {
    /// The address of a window
    Address(Address),
    /// A Regular Expression to match the window class (handled by Hyprland)
    ClassRegularExpression(&'a str),
    /// The window title
    Title(&'a str),
    /// The window's process Id
    ProcessId(u32),
}

/// This enum holds the fullscreen types
#[derive(Debug, Clone)]
pub enum FullscreenType {
    /// Fills the whole screen
    Real,
    /// Maximizes the window
    Maximize,
    /// Passes no param
    NoParam,
}

/// This enum holds directions, typically used for moving
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Direction {
    Up,
    Down,
    Right,
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

/// This enum holds a direction for cycling
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum CycleDirection {
    Next,
    Previous,
}

/// This enum is used for identifying monitors
#[derive(Debug, Clone)]
pub enum MonitorIdentifier<'a> {
    /// The monitor that is to the specified direction of the active one
    Direction(Direction),
    /// The monitor id
    Id(u8),
    /// The monitor name
    Name(&'a str),
    /// The current monitor
    Current,
    /// The workspace relative to the current workspace
    Relative(i32),
}

/// This enum holds corners
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Corner {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

/// This enum holds options that are applied to the current workspace
#[derive(Debug, Clone)]
pub enum WorkspaceOptions {
    /// Makes all windows pseudo tiled
    AllPseudo,
    /// Makes all windows float
    AllFloat,
}

/// This enum is for identifying workspaces that also includes the special workspace
#[derive(Debug, Clone)]
pub enum WorkspaceIdentifierWithSpecial<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    Relative(i32),
    /// The workspace on the monitor relative to the current monitor
    RelativeMonitor(i32),
    /// The open workspace relative to the current workspace
    RelativeOpen(i32),
    /// The previous Workspace
    Previous,
    /// The first available empty workspace
    Empty,
    /// The name of the workspace
    Name(&'a str),
    /// The special workspace
    Special(Option<&'a str>),
}

/// This enum is for identifying workspaces
#[derive(Debug, Clone)]
pub enum WorkspaceIdentifier<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    Relative(i32),
    /// The workspace on the monitor relative to the current monitor
    RelativeMonitor(i32),
    /// The open workspace relative to the current workspace
    RelativeOpen(i32),
    /// The previous Workspace
    Previous,
    /// The first available empty workspace
    Empty,
    /// The name of the workspace
    Name(&'a str),
}

/// This enum is the params to MoveWindow dispatcher
#[derive(Debug, Clone)]
pub enum WindowMove<'a> {
    /// Moves the window to a specified monitor
    Monitor(MonitorIdentifier<'a>),
    /// Moves the window in a specified direction
    Direction(Direction),
}

/// This enum holds every dispatcher
#[derive(Debug, Clone)]
pub enum DispatchType<'a> {
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
    /// This dispatcher kills the active window/client
    KillActiveWindow,
    /// This dispatcher closes the specified window
    CloseWindow(WindowIdentifier<'a>),
    /// This dispatcher changes the current workspace
    Workspace(WorkspaceIdentifierWithSpecial<'a>),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// changes the active workspace aswell
    MoveFocusedWindowToWorkspace(WorkspaceIdentifier<'a>),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// does not change workspaces
    MoveFocusedWindowToWorkspaceSilent(WorkspaceIdentifier<'a>),
    /// This dispatcher floats the current window
    ToggleFloating,
    /// This toggles the current window fullscreen state
    ToggleFullscreen(FullscreenType),
    /// This dispatcher sets the DPMS status for all monitors
    ToggleDPMS(bool, Option<&'a str>),
    /// This dispatcher toggles pseudo tiling for the current window
    TogglePseudo,
    /// This dispatcher pins the active window to all workspaces
    TogglePin,
    /// This dispatcher moves the window focus in a specified direction
    MoveFocus(Direction),
    /// This dispatcher moves the current window to a monitor or in a specified direction
    MoveWindow(WindowMove<'a>),
    /// This dispatcher centers the active window
    CenterWindow,
    /// This dispatcher resizes the active window using a [`Position`][Position] enum
    ResizeActive(Position),
    /// This dispatcher moves the active window using a [`Position`][Position] enum
    MoveActive(Position),
    /// This dispatcher resizes the specified window using a [`Position`][Position] enum
    ResizeWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher moves the specified window using a [`Position`][Position] enum
    MoveWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher cycles windows using a specified direction
    CycleWindow(CycleDirection),
    /// This dispatcher swaps windows using a specified direction
    SwapWindow(CycleDirection),
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
    /// This dispatcher applied a option to all windows in a workspace
    WorkspaceOption(WorkspaceOptions),
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
    ToggleSpecialWorkspace,
}

fn format_relative<T: Ord + std::fmt::Display + num_traits::Signed>(
    int: T,
    extra: &'_ str,
) -> String {
    if int.is_positive() {
        format!("{extra}+{int}")
    } else if int.is_negative() {
        format!("{extra}-{int}", int = int.abs())
    } else {
        "+0".to_string()
    }
}

fn match_workspace_identifier(identifier: WorkspaceIdentifier) -> String {
    use WorkspaceIdentifier::*;
    match identifier {
        Id(id) => format!("{id}"),
        Name(name) => format!("name:{name}"),
        Relative(int) => format_relative(int, ""),
        RelativeMonitor(int) => format_relative(int, "m"),
        RelativeOpen(int) => format_relative(int, "e"),
        Previous => "previous".to_string(),
        Empty => "empty".to_string(),
    }
}

fn match_workspace_identifier_special(identifier: WorkspaceIdentifierWithSpecial) -> String {
    use WorkspaceIdentifierWithSpecial::*;
    match identifier {
        Id(id) => format!("{id}"),
        Name(name) => format!("name:{name}"),
        Relative(int) => format_relative(int, ""),
        RelativeMonitor(int) => format_relative(int, "m"),
        RelativeOpen(int) => format_relative(int, "e"),
        Previous => "previous".to_string(),
        Empty => "empty".to_string(),
        Special(opt) => match opt {
            Some(name) => format!("special:{name}"),
            None => "special".to_string(),
        },
    }
}

fn match_mon_identifier(identifier: MonitorIdentifier) -> String {
    match identifier {
        MonitorIdentifier::Direction(dir) => match_dir(dir),
        MonitorIdentifier::Id(id) => id.to_string(),
        MonitorIdentifier::Name(name) => name.to_string(),
        MonitorIdentifier::Current => "current".to_string(),
        MonitorIdentifier::Relative(int) => format_relative(int, ""),
    }
}

fn match_dir(dir: Direction) -> String {
    match dir {
        Direction::Left => "l",
        Direction::Right => "r",
        Direction::Down => "d",
        Direction::Up => "u",
    }
    .to_string()
}

fn position_to_string(pos: Position) -> String {
    match pos {
        Position::Delta(x, y) => format!("{x},{y}"),
        Position::Exact(w, h) => format!("exact {w} {h}"),
    }
}

fn match_window_identifier(iden: WindowIdentifier) -> String {
    match iden {
        WindowIdentifier::Address(addr) => format!("address:{addr}"),
        WindowIdentifier::ProcessId(id) => format!("pid:{id}"),
        WindowIdentifier::ClassRegularExpression(regex) => regex.to_string(),
        WindowIdentifier::Title(title) => format!("title:{title}"),
    }
}

pub(crate) fn gen_dispatch_str(cmd: DispatchType, dispatch: bool) -> HResult<String> {
    use DispatchType::*;
    let sep = if dispatch { " " } else { "," };
    let string_to_pass = match &cmd {
        Exec(sh) => format!("exec{sep}{sh}"),
        Pass(win) => format!("pass{sep}{}", match_window_identifier(win.clone())),
        KillActiveWindow => "killactive".to_string(),
        CloseWindow(win) => {
            format!("closewindow{sep}{}", match_window_identifier(win.clone()))
        }
        Workspace(ident) => format!(
            "workspace{sep}{}",
            match_workspace_identifier_special(ident.clone())
        ),
        MoveFocusedWindowToWorkspace(ident) => {
            format!(
                "workspace{sep}{}",
                match_workspace_identifier(ident.clone())
            )
        }
        MoveFocusedWindowToWorkspaceSilent(ident) => {
            format!(
                "workspace{sep}{}",
                match_workspace_identifier(ident.clone())
            )
        }
        ToggleFloating => "togglefloating".to_string(),
        ToggleFullscreen(fullscreen_type) => format!(
            "fullscreen{sep}{}",
            match fullscreen_type {
                FullscreenType::Real => "0",
                FullscreenType::Maximize => "1",
                FullscreenType::NoParam => "",
            }
        ),
        ToggleDPMS(stat, mon) => {
            format!(
                "dpms{sep}{} {}",
                if *stat { "on" } else { "off" },
                match mon {
                    Some(s) => s,
                    None => "",
                }
            )
        }
        TogglePseudo => "pseudo".to_string(),
        TogglePin => "pin".to_string(),
        MoveFocus(dir) => format!(
            "movefocus{sep}{}",
            match dir {
                Direction::Down => "d",
                Direction::Up => "u",
                Direction::Right => "r",
                Direction::Left => "l",
            }
        ),
        MoveWindow(ident) => format!(
            "movewindow{sep}{}",
            match ident {
                WindowMove::Direction(dir) => match_dir(dir.clone()),
                WindowMove::Monitor(mon) => format!("mon:{}", match_mon_identifier(mon.clone())),
            }
        ),
        CenterWindow => "centerwindow".to_string(),
        ResizeActive(pos) => {
            format!("resizeactive{sep}{}", position_to_string(pos.clone()))
        }
        MoveActive(pos) => format!("moveactive {}", position_to_string(pos.clone())),
        ResizeWindowPixel(pos, win) => {
            format!(
                "resizeactive{sep}{} {}",
                position_to_string(pos.clone()),
                match_window_identifier(win.clone())
            )
        }
        MoveWindowPixel(pos, win) => format!(
            "moveactive{sep}{} {}",
            position_to_string(pos.clone()),
            match_window_identifier(win.clone())
        ),
        CycleWindow(dir) => format!(
            "cyclenext{sep}{}",
            match dir {
                CycleDirection::Next => "",
                CycleDirection::Previous => "prev",
            }
        ),
        SwapWindow(dir) => format!(
            "swapnext{sep}{}",
            match dir {
                CycleDirection::Next => "",
                CycleDirection::Previous => "prev",
            }
        ),
        FocusWindow(win) => {
            format!("focuswindow{sep}{}", match_window_identifier(win.clone()))
        }
        FocusMonitor(mon) => {
            format!("focusmonitor{sep}{}", match_mon_identifier(mon.clone()))
        }
        ChangeSplitRatio(ratio) => format!("splitratio {}", ratio),
        ToggleOpaque => "toggleopaque".to_string(),
        MoveCursorToCorner(corner) => format!(
            "movecursortocorner{sep}{}",
            match corner {
                Corner::BottomLeft => "0",
                Corner::BottomRight => "1",
                Corner::TopRight => "2",
                Corner::TopLeft => "3",
            }
        ),
        WorkspaceOption(opt) => format!(
            "workspaceopt{sep}{}",
            match opt {
                WorkspaceOptions::AllFloat => "allfloat",
                WorkspaceOptions::AllPseudo => "allpseudo",
            }
        ),
        Exit => "exit".to_string(),
        ForceRendererReload => "forcerendererreload".to_string(),
        MoveCurrentWorkspaceToMonitor(mon) => {
            format!(
                "movecurrentworkspacetomonitor{sep}{}",
                match_mon_identifier(mon.clone())
            )
        }
        MoveWorkspaceToMonitor(work, mon) => format!(
            "movecurrentworkspacetomonitor{sep}{} {}",
            match_workspace_identifier(work.clone()),
            match_mon_identifier(mon.clone())
        ),
        ToggleSpecialWorkspace => "togglespecialworkspace".to_string(),
        SwapActiveWorkspaces(mon, mon2) => format!(
            "swapactiveworkspaces{sep}{} {}",
            match_mon_identifier(mon.clone()),
            match_mon_identifier(mon2.clone())
        ),
        BringActiveToTop => "bringactivetotop".to_string(),
        SetCursor(theme, size) => {
            format!("{theme} {size}", size = *size)
        }
    };
    if let SetCursor(_, _) = cmd {
        Ok(format!("setcursor {string_to_pass}"))
    } else if dispatch {
        Ok(format!("dispatch {string_to_pass}"))
    } else {
        Ok(string_to_pass)
    }
}

/// The struct that provides all dispatching methods
pub struct Dispatch;

impl Dispatch {
    /// This function calls a specified dispatcher (blocking)
    ///
    /// ```rust
    /// # use hyprland::shared::HResult;
    /// # fn main() -> HResult<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call(DispatchType::Exec("kitty"))
    /// # }
    /// ```
    pub fn call(dispatch_type: DispatchType) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let output = write_to_socket_sync(
            socket_path,
            gen_dispatch_str(dispatch_type, true)?.as_bytes(),
        );

        match output {
        Ok(msg) => match msg.as_str() {
            "ok" => Ok(()),
            msg => panic!(
                "Hyprland returned a non `ok` value to the dispatcher, this is usually a error, output:({msg})"
            ),
        },
        Err(error) => panic!("A error occured when running the dispatcher: {error:#?}"),
    }
    }

    /// This function calls a specified dispatcher (async)
    ///
    /// ```rust
    /// # use hyprland::shared::HResult;
    /// # async fn function() -> HResult<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call_async(DispatchType::Exec("kitty")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_async(dispatch_type: DispatchType<'_>) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let output = write_to_socket(
            socket_path,
            gen_dispatch_str(dispatch_type, true)?.as_bytes(),
        )
        .await;

        match output {
        Ok(msg) => match msg.as_str() {
            "ok" => Ok(()),
            msg => panic!(
                "Hyprland returned a non `ok` value to the dispatcher, this is usually a error, output:({msg})"
            ),
        },
        Err(error) => panic!("A error occured when running the dispatcher: {error:#?}"),
    }
    }
}

/// Macro abstraction over [Dispatch::call]
#[macro_export]
macro_rules! dispatch {
    ($dis:ident, $( $arg:expr ), *) => {
        Dispatch::call(DispatchType::$dis($($arg), *))
    };
    (async; $dis:ident, $( $arg:expr ), *) => {
        Dispatch::call_async(DispatchType::$dis($($arg), *))
    };
}
