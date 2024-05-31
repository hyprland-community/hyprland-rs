/// Demonstrates usage of various dispatch calls
///
/// Usage: cargo run --example dispatch

use hyprland::dispatch::*;
use hyprland::shared::HyprError;
use hyprland::Result;

fn do_call(desc: &str, action: DispatchType) -> hyprland::Result<()> {
    println!("{}: {:?}", desc, action); 
    Dispatch::call(action)?; 
    std::thread::sleep(std::time::Duration::from_secs(2)); 
    return Ok(());
}

fn do_toggle(desc: &str, action: DispatchType) -> hyprland::Result<()> { 
    do_call(desc, action.clone())?; 
    do_call(desc, action)?;
    return Ok(());
}

fn main() -> hyprland::Result<()>{
    do_call("Moving cursor to top left", DispatchType::MoveCursorToCorner(Corner::TopLeft))?;
    do_call("Moving cursor to top right", DispatchType::MoveCursorToCorner(Corner::TopRight))?;
    do_call("Moving cursor to bottom right", DispatchType::MoveCursorToCorner(Corner::BottomRight))?;
    do_call("Moving cursor to bottom left", DispatchType::MoveCursorToCorner(Corner::BottomLeft))?;
    do_call("Moving window to next workspace", DispatchType::MoveToWorkspace(WorkspaceIdentifierWithSpecial::Relative(1), None))?;
    do_call("Moving window to previous workspace", DispatchType::MoveToWorkspace(WorkspaceIdentifierWithSpecial::Relative(-1), None))?;
    do_toggle("Toggling fullsceen", DispatchType::ToggleFullscreen(FullscreenType::Maximize))?;
    do_toggle("Toggling floating window", DispatchType::ToggleFloating(None))?;
    do_toggle("Toggling split layout", DispatchType::ToggleSplit)?;
    do_toggle("Toggling opaque", DispatchType::ToggleOpaque)?;
    return Ok(());
}   