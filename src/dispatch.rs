//! # Dispatch module
//!
//! This module is used for calling dispatchers and changing keywords
//!
//! ## Usage
//!
//! ```rust
//! use hyprland::dispatch::{dispatch_blocking, DispatchType};
//! fn main() -> std::io::Result<()> {
//!    dispatch_blocking(DispatchType::Exec("kitty".to_string()))?;
//!
//!    Ok(())
//! }
//! ````

use crate::shared::{get_socket_path, write_to_socket, Address, SocketType, WorkspaceId};
use std::io;
use tokio::runtime::Runtime;

/// This enum is for identifying a window
#[derive(Clone)]
pub enum WindowIdentifier {
    /// The address of a window
    Address(Address),
    /// A Regular Expression to match the window class (handled by Hyprland)
    ClassRegularExpression(String),
    /// The window title
    Title(String),
    /// The window's process Id
    ProcessId(u32),
}

/// This enum holds the fullscreen types
pub enum FullscreenType {
    /// Fills the whole screen
    Real,
    /// Maximizes the window
    Maximize,
}

/// This enum holds directions, typically used for moving
#[derive(Clone)]
#[allow(missing_docs)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

/// This enum is used for resizing and moving windows precisely
#[derive(Clone)]
pub enum Position {
    /// A delta
    Delta(i16, i16),
    /// The exact size
    Exact(i16, i16),
}

/// This enum holds a direction for cycling
#[allow(missing_docs)]
pub enum CycleDirection {
    Next,
    Previous,
}

/// This enum is used for identifying monitors
#[derive(Clone)]
pub enum MonitorIdentifier {
    /// The monitor that is to the specified direction of the active one
    Direction(Direction),
    /// The monitor id
    Id(u8),
    /// The monitor name
    Name(String),
}

/// This enum holds corners
#[allow(missing_docs)]
pub enum Corner {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

/// This enum holds options that are applied to the current workspace
pub enum WorkspaceOptions {
    /// Makes all windows pseudo tiled
    AllPseudo,
    /// Makes all windows float
    AllFloat,
}

/// This enum is for identifying workspaces that also includes the special workspace
#[derive(Clone)]
pub enum WorkspaceIdentifierWithSpecial {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace (positive)
    PositiveRelative(u8),
    /// The workspace relative to the current workspace (positive)
    NegativeRelative(u8),
    /// The workspace on the monitor relative to the current monitor (positive)
    PositiveRelativeMonitor(u8),
    /// The workspace on the monitor relative to the current monitor (negative)
    NegativeRelativeMonitor(u8),
    /// The name of the workspace
    Name(String),
    /// The special workspace
    Special,
}

/// This enum is for identifying workspaces
#[derive(Clone)]
pub enum WorkspaceIdentifier {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace (positive)
    PositiveRelative(u8),
    /// The workspace relative to the current workspace (positive)
    NegativeRelative(u8),
    /// The workspace on the monitor relative to the current monitor (positive)
    PositiveRelativeMonitor(u8),
    /// The workspace on the monitor relative to the current monitor (negative)
    NegativeRelativeMonitor(u8),
    /// The name of the workspace
    Name(String),
}

/// This enum is the params to MoveWindow dispatcher
pub enum WindowMove {
    /// Moves the window to a specified monitor
    Monitor(MonitorIdentifier),
    /// Moves the window in a specified direction
    Direction(Direction),
}

/// This enum holds every dispatcher
pub enum DispatchType {
    /// This dispatcher changes a keyword
    Keyword(String, String),
    /// This dispatcher executes a program
    Exec(String),
    /// This dispatcher kills the active window/client
    KillActiveWindow,
    /// This dispatcher changes the current workspace
    Workspace(WorkspaceIdentifierWithSpecial),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// changes the active workspace aswell
    MoveFocusedWindowToWorkspace(WorkspaceIdentifier),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// does not change workspaces
    MoveFocusedWindowToWorkspaceSilent(WorkspaceIdentifier),
    /// This dispatcher floats the current window
    ToggleFloating,
    /// This toggles the current window fullscreen state
    ToggleFullscreen(FullscreenType),
    /// This dispatcher toggles pseudo tiling for the current window
    TogglePseudo,
    /// This dispatcher moves the window focus in a specified direction
    MoveFocus(Direction),
    /// This dispatcher moves the current window to a monitor or in a specified direction
    MoveWindow(WindowMove),
    /// This dispatcher resizes the active window using a [`Position`][Position] enum
    ResizeActive(Position),
    /// This dispatcher moves the active window using a [`Position`][Position] enum
    MoveActive(Position),
    /// This dispatcher cycles windows using a specified direction
    CycleWindow(CycleDirection),
    /// This dispatcher focuses a specified window
    FocusWindow(WindowIdentifier),
    /// This dispatcher focuses a specified monitor
    FocusMonitor(MonitorIdentifier),
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
    MoveCurrentWorkspaceToMonitor(MonitorIdentifier),
    /// This dispatcher moves a specified workspace to a specified monitor
    MoveWorkspaceToMonitor(WorkspaceIdentifier, MonitorIdentifier),
    /// This toggles the special workspace (AKA scratchpad)
    ToggleSpecialWorkspace,
}

fn match_workspace_identifier(identifier: WorkspaceIdentifier) -> String {
    match identifier {
        WorkspaceIdentifier::Id(id) => format!("{id}"),
        WorkspaceIdentifier::Name(name) => format!("name:{name}"),
        WorkspaceIdentifier::PositiveRelative(int) => format!("+{int}"),
        WorkspaceIdentifier::PositiveRelativeMonitor(int) => format!("m+{int}"),
        WorkspaceIdentifier::NegativeRelative(int) => format!("-{int}"),
        WorkspaceIdentifier::NegativeRelativeMonitor(int) => format!("m-{int}"),
    }
}

fn match_workspace_identifier_special(identifier: WorkspaceIdentifierWithSpecial) -> String {
    match identifier {
        WorkspaceIdentifierWithSpecial::Id(id) => format!("{id}"),
        WorkspaceIdentifierWithSpecial::Name(name) => format!("name:{name}"),
        WorkspaceIdentifierWithSpecial::PositiveRelative(int) => format!("+{int}"),
        WorkspaceIdentifierWithSpecial::PositiveRelativeMonitor(int) => format!("m+{int}"),
        WorkspaceIdentifierWithSpecial::NegativeRelative(int) => format!("-{int}"),
        WorkspaceIdentifierWithSpecial::NegativeRelativeMonitor(int) => format!("m-{int}"),
        WorkspaceIdentifierWithSpecial::Special => "special".to_string(),
    }
}

fn match_mon_indentifier(identifier: MonitorIdentifier) -> String {
    match identifier {
        MonitorIdentifier::Direction(dir) => match_dir(dir),
        MonitorIdentifier::Id(id) => id.to_string(),
        MonitorIdentifier::Name(name) => name,
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
        WindowIdentifier::Address(addr) => format!("address:{}", addr),
        WindowIdentifier::ProcessId(id) => format!("pid:{}", id),
        WindowIdentifier::ClassRegularExpression(regex) => regex,
        WindowIdentifier::Title(title) => format!("title:{}", title),
    }
}

async fn dispatch_cmd(cmd: DispatchType) -> io::Result<String> {
    let socket_path = get_socket_path(SocketType::Command);
    let string_to_pass = match &cmd {
        DispatchType::Exec(sh) => format!("exec {sh}"),
        DispatchType::KillActiveWindow => "killactive".to_string(),
        DispatchType::Workspace(identifier) => format!(
            "workspace {}",
            match_workspace_identifier_special(identifier.clone())
        ),
        DispatchType::MoveFocusedWindowToWorkspace(identifier) => {
            format!(
                "workspace {}",
                match_workspace_identifier(identifier.clone())
            )
        }
        DispatchType::MoveFocusedWindowToWorkspaceSilent(identifier) => {
            format!(
                "workspace {}",
                match_workspace_identifier(identifier.clone())
            )
        }
        DispatchType::ToggleFloating => "togglefloating".to_string(),
        DispatchType::ToggleFullscreen(fullscreen_type) => format!(
            "fullscreen {}",
            match fullscreen_type {
                FullscreenType::Real => "0",
                FullscreenType::Maximize => "1",
            }
        ),
        DispatchType::TogglePseudo => "pseudo".to_string(),
        DispatchType::MoveFocus(dir) => format!(
            "movefocus {}",
            match dir {
                Direction::Down => "d",
                Direction::Up => "u",
                Direction::Right => "r",
                Direction::Left => "l",
            }
        ),
        DispatchType::MoveWindow(iden) => format!(
            "movewindow {}",
            match iden {
                WindowMove::Direction(dir) => match_dir(dir.clone()),
                WindowMove::Monitor(mon) => format!("mon:{}", match_mon_indentifier(mon.clone())),
            }
        ),
        DispatchType::ResizeActive(pos) => {
            format!("resizeactive {}", position_to_string(pos.clone()))
        }
        DispatchType::MoveActive(pos) => format!("moveactive {}", position_to_string(pos.clone())),
        DispatchType::CycleWindow(dir) => format!(
            "cyclenext {}",
            match dir {
                CycleDirection::Next => "",
                CycleDirection::Previous => "prev",
            }
        ),
        DispatchType::FocusWindow(win) => {
            format!("focuswindow {}", match_window_identifier(win.clone()))
        }
        DispatchType::FocusMonitor(mon) => {
            format!("focusmonitor {}", match_mon_indentifier(mon.clone()))
        }
        DispatchType::ChangeSplitRatio(ratio) => format!("splitratio {}", ratio),
        DispatchType::ToggleOpaque => "toggleopaque".to_string(),
        DispatchType::MoveCursorToCorner(corner) => format!(
            "movecursortocorner {}",
            match corner {
                Corner::BottomLeft => "0",
                Corner::BottomRight => "1",
                Corner::TopRight => "2",
                Corner::TopLeft => "3",
            }
        ),
        DispatchType::WorkspaceOption(opt) => format!(
            "workspaceopt {}",
            match opt {
                WorkspaceOptions::AllFloat => "allfloat",
                WorkspaceOptions::AllPseudo => "allpseudo",
            }
        ),
        DispatchType::Exit => "exit".to_string(),
        DispatchType::ForceRendererReload => "forcerendererreload".to_string(),
        DispatchType::MoveCurrentWorkspaceToMonitor(mon) => {
            format!(
                "movecurrentworkspacetomonitor {}",
                match_mon_indentifier(mon.clone())
            )
        }
        DispatchType::MoveWorkspaceToMonitor(work, mon) => format!(
            "movecurrentworkspacetomonitor {} {}",
            match_workspace_identifier(work.clone()),
            match_mon_indentifier(mon.clone())
        ),
        DispatchType::ToggleSpecialWorkspace => "togglespecialworkspace".to_string(),
        DispatchType::Keyword(key, val) => {
            format!("{key} {val}", key = key.clone(), val = val.clone())
        }
    };
    let output = if let DispatchType::Keyword(_, _) = cmd {
        write_to_socket(socket_path, format!("keyword {string_to_pass}").as_bytes()).await?
    } else {
        write_to_socket(socket_path, format!("dispatch {string_to_pass}").as_bytes()).await?
    };

    Ok(output)
}

/// This function calls a specified dispatcher (blocking)
/// 
/// ```rust
/// dispatch_blocking(DispatchType::SomeDispatcher)
/// ```
pub fn dispatch_blocking(dispatch_type: DispatchType) -> io::Result<()> {
    let rt = Runtime::new()?;
    match rt.block_on(dispatch_cmd(dispatch_type)) {
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
/// dispatch(DispatchType::SomeDispatcher).await
/// ```
pub async fn dispatch(dispatch_type: DispatchType) -> io::Result<()> {
    match dispatch_cmd(dispatch_type).await {
        Ok(msg) => match msg.as_str() {
            "ok" => Ok(()),
            msg => panic!(
                "Hyprland returned a non `ok` value to the dispatcher, this is usually a error, output:({msg})"
            ),
        },
        Err(error) => panic!("A error occured when running the dispatcher: {error:#?}"),
    }
}
