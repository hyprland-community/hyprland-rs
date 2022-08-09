use crate::shared::{get_socket_path, write_to_socket, SocketType};
use serde_json::Result;
use std::io;

use crate::data::shared::*;

/// This private function is to call socket commands
async fn call_hyprctl_data_cmd(cmd: DataCommands) -> io::Result<String> {
    let cmd_string = match cmd {
        DataCommands::Monitors => "monitors",
        DataCommands::ActiveWindow => "activewindow",
        DataCommands::Clients => "clients",
        DataCommands::Devices => "devices",
        DataCommands::Layers => "layers",
        DataCommands::Workspaces => "workspaces",
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
pub async fn get_active_window() -> Result<Client> {
    let data = match call_hyprctl_data_cmd(DataCommands::ActiveWindow).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let deserialized: Client = serde_json::from_str(&data)?;
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
