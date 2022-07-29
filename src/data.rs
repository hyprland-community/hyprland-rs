//mod shared;

use crate::shared::{get_socket_path, write_to_socket, SocketType, HyprAddress};
use std::io;

use serde::{Deserialize, Serialize};
use serde_json::Result;
extern crate hex;

use std::collections::HashMap;

#[derive(Debug)]
pub enum HyprCtlDataCommands {
    Monitors,
    Workspaces,
    Clients,
    ActiveWindow,
    Layers,
    Devices,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprWorkspaceBasic {
    id: u8,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprMonitor {
    id: u8,
    name: String,
    width: u16,
    height: u16,
    #[serde(rename = "refreshRate")]
    refresh_rate: f32,
    x: i32,
    y: i32,
    #[serde(rename = "activeWorkspace")]
    active_workspace: HyprWorkspaceBasic,
    reserved: (u8, u8, u8, u8),
    scale: f32,
    transform: i16,
    active: String, // TODO make into bool somehow
}

pub type HyprMonitors = Vec<HyprMonitor>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprWorkspace {
    id: u8,
    name: String,
    monitor: String,
    windows: u8,
    hasfullscreen: u8,
}

pub type HyprWorkspaces = Vec<HyprWorkspace>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprClient {
    address: HyprAddress,
    at: (i16, i16),
    size: (u16, u16),
    workspace: HyprWorkspaceBasic,
    floating: u8,
    monitor: u8,
    class: String,
    title: String,
    pid: u32,
}

pub type HyprClients = Vec<HyprClient>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprActiveWindow {
    address: HyprAddress,
    at: (i32, i32),
    size: (u16, u16),
    workspace: HyprWorkspaceBasic,
    floating: u8,
    monitor: u8,
    class: String,
    title: String,
    pid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprLayerClient {
    address: HyprAddress,
    x: i32,
    y: i32,
    w: u16,
    h: u16,
    namespace: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprLayerDisplay {
    levels: HashMap<String, Vec<HyprLayerClient>>,
}

pub type HyprLayers = HashMap<String, HyprLayerDisplay>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprMouse {
    address: HyprAddress,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprKeyboard {
    address: HyprAddress,
    name: String,
    rules: String,
    model: String,
    layout: String,
    variant: String,
    options: String,
    active_keymap: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprTablet {
    address: HyprAddress,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprDevices {
    mice: Vec<HyprMouse>,
    keyboards: Vec<HyprKeyboard>,
    tablets: Vec<HyprTablet>,
}

fn call_hyprctl_data_cmd(cmd: HyprCtlDataCommands) -> io::Result<String> {
    use tokio::runtime::Runtime;

    let cmd_string = match cmd {
        HyprCtlDataCommands::Monitors => "monitors",
        HyprCtlDataCommands::ActiveWindow => "activewindow",
        HyprCtlDataCommands::Clients => "clients",
        HyprCtlDataCommands::Devices => "devices",
        HyprCtlDataCommands::Layers => "layers",
        HyprCtlDataCommands::Workspaces => "workspaces",
    };

    let socket_path = get_socket_path(SocketType::Command);

    let rt = Runtime::new()?;

    rt.block_on(write_to_socket(
        socket_path,
        format!("j/{cmd_string}").as_bytes(),
    ))
}

pub fn get_monitors() -> Result<HyprMonitors> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::Monitors) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprMonitors = serde_json::from_str(&data)?;
    Ok(serialized)
}

pub fn get_workspaces() -> Result<HyprWorkspaces> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::Workspaces) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprWorkspaces = serde_json::from_str(&data)?;
    Ok(serialized)
}

pub fn get_clients() -> Result<HyprClients> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::Clients) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprClients = serde_json::from_str(&data)?;
    Ok(serialized)
}

pub fn get_active_window() -> Result<HyprActiveWindow> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::ActiveWindow) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprActiveWindow = serde_json::from_str(&data)?;
    Ok(serialized)
}

pub fn get_layers() -> Result<HyprLayers> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::Layers) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprLayers = serde_json::from_str(&data)?;
    Ok(serialized)
}

pub fn get_devices() -> Result<HyprDevices> {
    let data = match call_hyprctl_data_cmd(HyprCtlDataCommands::Devices) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    };
    let serialized: HyprDevices = serde_json::from_str(&data)?;
    Ok(serialized)
}
