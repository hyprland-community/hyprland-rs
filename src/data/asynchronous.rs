use crate::shared::{get_socket_path, write_to_socket, SocketType};
use serde_json::Result;
use std::io;

use crate::data::shared::*;

/// This private function is to call socket commands
async fn call_hyprctl_data_cmd(cmd: DataCommands) -> io::Result<String> {
    let cmd_string = match cmd {
        DataCommands::Monitors => "monitors".to_string(),
        DataCommands::ActiveWindow => "activewindow".to_string(),
        DataCommands::Clients => "clients".to_string(),
        DataCommands::Devices => "devices".to_string(),
        DataCommands::Layers => "layers".to_string(),
        DataCommands::Workspaces => "workspaces".to_string(),
        DataCommands::Version => "version".to_string(),
        DataCommands::Keyword(key) => format!("getoption {key}"),
    };

    let socket_path = get_socket_path(SocketType::Command);

    write_to_socket(socket_path, format!("j/{cmd_string}").as_bytes()).await
}

/// This function returns all monitors
pub async fn get_monitors() -> Result<Monitors> {
    let data = match call_hyprctl_data_cmd(DataCommands::Monitors).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Monitors = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns all workspaces
pub async fn get_workspaces() -> Result<Workspaces> {
    let data = match call_hyprctl_data_cmd(DataCommands::Workspaces).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Workspaces = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns all clients/windows
pub async fn get_clients() -> Result<Clients> {
    let data = match call_hyprctl_data_cmd(DataCommands::Clients).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Clients = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns the active window
pub async fn get_active_window() -> Result<ActiveWindow> {
    let data = match call_hyprctl_data_cmd(DataCommands::ActiveWindow).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: ActiveWindow = serde_json::from_str(&data)?;
    Ok(deserialized)
}
/// This function returns all layer surfaces
pub async fn get_layers() -> Result<Layers> {
    let data = match call_hyprctl_data_cmd(DataCommands::Layers).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Layers = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns all devices (mice, keyboards, tablets)
pub async fn get_devices() -> Result<Devices> {
    let data = match call_hyprctl_data_cmd(DataCommands::Devices).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Devices = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns Hyprland version details
pub async fn get_version() -> Result<Version> {
    let data = match call_hyprctl_data_cmd(DataCommands::Version).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Version = serde_json::from_str(&data)?;
    Ok(deserialized)
}

/// This function returns data about a keyword
pub async fn get_keyword(key: String) -> Result<Keyword> {
    let data = match call_hyprctl_data_cmd(DataCommands::Keyword(key)).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: OptionRaw = serde_json::from_str(&data)?;
    let dc = deserialized.clone();
    let keyword = Keyword {
        option: deserialized.option,
        value: if deserialized.int != -1 {
            OptionValue::Int(deserialized.int)
        } else if deserialized.float != -1.0 {
            OptionValue::Float(deserialized.float)
        } else if deserialized.str != *"".to_string() {
            OptionValue::String(deserialized.str)
        } else {
            panic!("The option returned data that was unrecognized: {dc:#?}")
        },
    };
    Ok(keyword)
}

/// A helper function to get the current workspace
pub async fn get_active_workspace() -> Result<Workspace> {
    let monitor = get_active_monitor().await?;
    let workspace_id = monitor.active_workspace.id;
    let workspaces = get_workspaces().await?;

    if let Some(work) = workspaces.iter().find(|item| item.id == workspace_id) {
        Ok(work.clone())
    } else {
        panic!("No active workspace?")
    }
}

/// A helper function to get the current monitor
pub async fn get_active_monitor() -> Result<Monitor> {
    let monitors = get_monitors().await?;
    if let Some(mon) = monitors.iter().find(|item| item.focused) {
        Ok(mon.clone())
    } else {
        panic!("No active monitor?")
    }
}

/// A helper function to get the current fullscreen state
pub async fn get_fullscreen_state() -> Result<bool> {
    let work = get_active_workspace().await?;
    Ok(work.hasfullscreen)
}
