use crate::data::shared::*;
use serde_json::Result;
use tokio::runtime::Runtime;

use crate::data::asynchronous;

lazy_static! {
    static ref RT: Runtime = match Runtime::new() {
        Ok(run) => run,
        Err(e) => panic!("Error creating tokio runtime: {e}"),
    };
}
/// This function returns all monitors
pub fn get_monitors() -> Result<Monitors> {
    RT.block_on(asynchronous::get_monitors())
}

/// This function returns all workspaces
pub fn get_workspaces() -> Result<Workspaces> {
    RT.block_on(asynchronous::get_workspaces())
}

/// This function returns all clients/windows
pub fn get_clients() -> Result<Clients> {
    RT.block_on(asynchronous::get_clients())
}

/// This function returns the active window
pub fn get_active_window() -> Result<Client> {
    RT.block_on(asynchronous::get_active_window())
}
/// This function returns all layer surfaces
pub fn get_layers() -> Result<Layers> {
    RT.block_on(asynchronous::get_layers())
}

/// This function returns all devices (mice, keyboards, tablets)
pub fn get_devices() -> Result<Devices> {
    RT.block_on(asynchronous::get_devices())
}

/// This function returns Hyprland version details
pub fn get_version() -> Result<Version> {
    RT.block_on(asynchronous::get_version())
}

/// This function returns data about a keyword
pub fn get_keyword(key: String) -> Result<Keyword> {
    RT.block_on(asynchronous::get_keyword(key))
}

/// A helper function to get the current monitor
pub fn get_active_monitor() -> Result<Monitor> {
    RT.block_on(asynchronous::get_active_monitor())
}

/// A helper function to get the current workspace
pub fn get_active_workspace() -> Result<Workspace> {
    RT.block_on(asynchronous::get_active_workspace())
}

/// This function returns all devices (mice, keyboards, tablets)
pub fn get_fullscreen_state() -> Result<bool> {
    RT.block_on(asynchronous::get_fullscreen_state())
}
