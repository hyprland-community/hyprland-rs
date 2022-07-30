use crate::shared::{get_socket_path, write_to_socket, Address, SocketType, WorkspaceId};
use std::io;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub enum WindowIdentifier {
    Address(Address),
    ClassRegularExpression(String),
    Title(String),
    ProcessId(u32),
}

pub enum FullscreenType {
    Real,
    Maximize,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
pub enum Position {
    Delta(i16, i16),
    Exact(u16, u16),
}

pub enum CycleDirection {
    Next,
    Previous,
}

#[derive(Clone)]
pub enum MonitorIdentifier {
    Direction(Direction),
    Id(u8),
    Name(String),
}

pub enum Corner {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

pub enum WorkspaceOptions {
    AllPseudo,
    AllFloat,
}

#[derive(Clone)]
pub enum WorkspaceIdentifierWithSpecial {
    Id(WorkspaceId),
    PositiveRelative(u8),
    NegativeRelative(u8),
    PositiveRelativeMonitor(u8),
    NegativeRelativeMonitor(u8),
    Name(String),
    Special,
}

#[derive(Clone)]
pub enum WorkspaceIdentifier {
    Id(WorkspaceId),
    PositiveRelative(u8),
    NegativeRelative(u8),
    PositiveRelativeMonitor(u8),
    NegativeRelativeMonitor(u8),
    Name(String),
}

pub enum WindowMove {
    Monitor(MonitorIdentifier),
    Direction(Direction),
}

pub enum DispatchType {
    Keyword(String, String),
    Exec(String),
    KillActiveWindow,
    Workspace(WorkspaceIdentifierWithSpecial),
    MoveFocusedWindowToWorkspace(WorkspaceIdentifier),
    MoveFocusedWindowToWorkspaceSilent(WorkspaceIdentifier),
    ToggleFloating,
    ToggleFullscreen(FullscreenType),
    TogglePseudo,
    MoveFocus(Direction),
    MoveWindow(WindowMove),
    ResizeActive(Position),
    MoveActive(Position),
    CycleWindow(CycleDirection),
    FocusWindow(WindowIdentifier),
    FocusMonitor(MonitorIdentifier),
    ChangeSplitRatio(f32),
    ToggleOpaque,
    MoveCursorToCorner(Corner),
    WorkspaceOption(WorkspaceOptions),
    Exit,
    ForceRendererReload,
    MoveCurrentWorkspaceToMonitor(MonitorIdentifier),
    MoveWorkspaceToMonitor(WorkspaceIdentifier, MonitorIdentifier),
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
        WindowIdentifier::Address(addr) => format!("address:{}", addr.to_string()),
        WindowIdentifier::ProcessId(id) => format!("pid:{}", id.to_string()),
        WindowIdentifier::ClassRegularExpression(regex) => format!("{}", regex),
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
            .to_string()
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
        DispatchType::ChangeSplitRatio(ratio) => format!("splitratio {}", ratio.to_string()),
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

pub fn dispatch(dispatch_type: DispatchType) -> io::Result<()> {
    let rt = Runtime::new()?;
    match rt.block_on(dispatch_cmd(dispatch_type)) {
        Ok(msg) => match msg.as_str() {
            "ok" => Ok(()),
            msg => panic!("a error has occured {msg}"),
        },
        Err(error) => panic!("error: {error:#?}"),
    }
}
