use crate::data::shared::*;
use serde_json::Result;
use tokio::runtime::Runtime;

use crate::data::asynchronous::{
    get_active_window as gaw, get_clients as gc, get_devices as gd, get_layers as gl,
    get_monitors as gm, get_workspaces as gw,
};

lazy_static! {
    static ref RT: Runtime = match Runtime::new() {
        Ok(run) => run,
        Err(e) => panic!("Error creating tokio runtime: {e}"),
    };
}
/// This function returns all monitors
pub fn get_monitors() -> Result<Monitors> {
    RT.block_on(gm())
}

/// This function returns all workspaces
pub fn get_workspaces() -> Result<Workspaces> {
    RT.block_on(gw())
}

/// This function returns all clients/windows
pub fn get_clients() -> Result<Clients> {
    RT.block_on(gc())
}

/// This function returns the active window
pub fn get_active_window() -> Result<Client> {
    RT.block_on(gaw())
}
/// This function returns all layer surfaces
pub fn get_layers() -> Result<Layers> {
    RT.block_on(gl())
}

/// This function returns all devices (mice, keyboards, tablets)
pub fn get_devices() -> Result<Devices> {
    RT.block_on(gd())
}
