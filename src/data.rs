//! # Data module
//! 
//! This module provides functions for getting information on the compositor
//!
//! ## Usage
//! 
//! here is a example of every function in use!
//! ```rust
//! use hyprland::data::{
//!     get_monitors,
//!     get_workspaces,
//!     get_clients,
//!     get_active_window,
//!     get_layers,
//!     get_devices
//! };
//!
//! fn main() -> std::io::Result<()> {
//!     let monitors = get_monitors()?;
//!     println!("{monitors:#?}");
//!
//!     let workspaces = get_workspaces()?;
//!     println!("{workspaces:#?}");
//!
//!     let clients = get_clients()?;
//!     println!("{clients:#?}");
//!
//!     let active_window = get_active_window()?;
//!     println!("{active_window:#?}");
//!
//!     let layers = get_layers()?;
//!     println!("{layers:#?}");
//!
//!     let devices = get_devices()?;
//!     println!("{devices:#?}");
//!
//!     Ok(())
//! }
//! ````

use crate::shared::{get_socket_path, write_to_socket, SocketType, Address};
use std::io;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

extern crate hex;

/// This internal enum holds every socket command that returns data
#[derive(Debug)]
enum DataCommands {
    Monitors,
    Workspaces,
    Clients,
    ActiveWindow,
    Layers,
    Devices,
}

/// This struct holds a basic identifier for a workspace often used in other structs
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceBasic {
    id: u8,
    name: String,
}

/// This struct holds information for a monitor
#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    id: u8,
    name: String,
    width: u16,
    height: u16,
    #[serde(rename = "refreshRate")]
    refresh_rate: f32,
    x: i32,
    y: i32,
    #[serde(rename = "activeWorkspace")]
    active_workspace: WorkspaceBasic,
    reserved: (u8, u8, u8, u8),
    scale: f32,
    transform: i16,
    active: String, // TODO make into bool somehow
}

/// This type provides a vector of monitors
pub type Monitors = Vec<Monitor>;

/// This struct holds information for a workspace
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    id: u8,
    name: String,
    monitor: String,
    windows: u8,
    hasfullscreen: u8,
}

/// This type provides a vector of workspaces
pub type Workspaces = Vec<Workspace>;

/// This struct holds information for a client/window
#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    address: Address,
    at: (i16, i16),
    size: (u16, u16),
    workspace: WorkspaceBasic,
    floating: u8,
    monitor: u8,
    class: String,
    title: String,
    pid: u32,
}

/// This type provides a vector of clients
pub type Clients = Vec<Client>;

/// This struct holds information about the active window/client 
#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveWindow {
    address: Address,
    at: (i32, i32),
    size: (u16, u16),
    workspace: WorkspaceBasic,
    floating: u8,
    monitor: u8,
    class: String,
    title: String,
    pid: u32,
}

/// This struct holds information about a layer surface/client
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerClient {
    address: Address,
    x: i32,
    y: i32,
    w: u16,
    h: u16,
    namespace: String,
}

/// This struct holds all the layer surfaces for a display
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerDisplay {
    levels: HashMap<String, Vec<LayerClient>>,
}

/// This type provides a hashmap of all current displays, and their layer surfaces
pub type Layers = HashMap<String, LayerDisplay>;

/// This struct holds information about a mouse device
#[derive(Serialize, Deserialize, Debug)]
pub struct Mouse {
    address: Address,
    name: String,
}

/// This struct holds information about a keyboard device
#[derive(Serialize, Deserialize, Debug)]
pub struct Keyboard {
    address: Address,
    name: String,
    rules: String,
    model: String,
    layout: String,
    variant: String,
    options: String,
    active_keymap: String,
}

/// This struct holds information about a tablet device
#[derive(Serialize, Deserialize, Debug)]
pub struct Tablet {
    address: Address,
    name: String,
}

/// This struct holds all current devices
#[derive(Serialize, Deserialize, Debug)]
pub struct Devices {
    mice: Vec<Mouse>,
    keyboards: Vec<Keyboard>,
    tablets: Vec<Tablet>,
}

/// This internal function is to call socket commands
fn call_hyprctl_data_cmd(cmd: DataCommands) -> io::Result<String> {
    use tokio::runtime::Runtime;

    let cmd_string = match cmd {
        DataCommands::Monitors => "monitors",
        DataCommands::ActiveWindow => "activewindow",
        DataCommands::Clients => "clients",
        DataCommands::Devices => "devices",
        DataCommands::Layers => "layers",
        DataCommands::Workspaces => "workspaces",
    };

    let socket_path = get_socket_path(SocketType::Command);

    let rt = Runtime::new()?;

    rt.block_on(write_to_socket(
        socket_path,
        format!("j/{cmd_string}").as_bytes(),
    ))
}

/// This function returns all monitors
pub fn get_monitors() -> Result<Monitors> {
    let data = match call_hyprctl_data_cmd(DataCommands::Monitors) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: Monitors = serde_json::from_str(&data)?;
    Ok(serialized)
}

/// This function returns all workspaces
pub fn get_workspaces() -> Result<Workspaces> {
    let data = match call_hyprctl_data_cmd(DataCommands::Workspaces) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: Workspaces = serde_json::from_str(&data)?;
    Ok(serialized)
}

/// This function returns all clients/windows
pub fn get_clients() -> Result<Clients> {
    let data = match call_hyprctl_data_cmd(DataCommands::Clients) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: Clients = serde_json::from_str(&data)?;
    Ok(serialized)
}

/// This function returns the active window
pub fn get_active_window() -> Result<ActiveWindow> {
    let data = match call_hyprctl_data_cmd(DataCommands::ActiveWindow) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: ActiveWindow = serde_json::from_str(&data)?;
    Ok(serialized)
}
/// This function returns all layer surfaces
pub fn get_layers() -> Result<Layers> {
    let data = match call_hyprctl_data_cmd(DataCommands::Layers) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: Layers = serde_json::from_str(&data)?;
    Ok(serialized)
}

/// This function returns all devices (mice, keyboards, tablets)
pub fn get_devices() -> Result<Devices> {
    let data = match call_hyprctl_data_cmd(DataCommands::Devices) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: Devices = serde_json::from_str(&data)?;
    Ok(serialized)
}
