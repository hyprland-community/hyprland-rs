/// Demonstrates usage of various asyncronous dispatch calls
///
/// Usage: cargo run --example dispatch_async

use hyprland::dispatch::*;

async fn do_call(desc: &str, action: DispatchType<'_>) -> hyprland::Result<()> {
    println!("{}: {:?}", desc, action); 
    Dispatch::call_async(action).await?; 
    std::thread::sleep(std::time::Duration::from_secs(2)); 
    return Ok(());
}

async fn do_toggle(desc: &str, action: DispatchType<'_>) -> hyprland::Result<()> { 
    do_call(desc, action.clone()).await?; 
    do_call(desc, action).await?;
    return Ok(());
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> hyprland::Result<()>{
    do_call("Moving cursor to top left", DispatchType::MoveCursorToCorner(Corner::TopLeft)).await?;
    do_call("Moving cursor to top right", DispatchType::MoveCursorToCorner(Corner::TopRight)).await?;
    do_call("Moving cursor to bottom right", DispatchType::MoveCursorToCorner(Corner::BottomRight)).await?;
    do_call("Moving cursor to bottom left", DispatchType::MoveCursorToCorner(Corner::BottomLeft)).await?;
    do_call("Moving window to next workspace", DispatchType::MoveToWorkspace(WorkspaceIdentifierWithSpecial::Relative(1), None)).await?;
    do_call("Moving window to previous workspace", DispatchType::MoveToWorkspace(WorkspaceIdentifierWithSpecial::Relative(-1), None)).await?;
    do_toggle("Toggling fullsceen", DispatchType::ToggleFullscreen(FullscreenType::Maximize)).await?;
    do_toggle("Toggling floating window", DispatchType::ToggleFloating(None)).await?;
    do_toggle("Toggling split layout", DispatchType::ToggleSplit).await?;
    do_toggle("Toggling opaque", DispatchType::ToggleOpaque).await?;
    return Ok(());
}   