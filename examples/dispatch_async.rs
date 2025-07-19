/// Demonstrates usage of various asyncronous dispatch calls
///
/// Usage: cargo run --example dispatch_async
use hyprland::{default_instance_panic, dispatch};
use hyprland::dispatch::DispatchType::*;
use hyprland::dispatch::{Corner, Dispatch, FullscreenType, WorkspaceIdentifierWithSpecial};
use hyprland::instance::Instance;


fn describe(desc: &str) {
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{desc}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
    let program = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    let instance = default_instance_panic();

    println!("Executing {program}");
    dispatch!(async; instance, Exec, &program).await?;

    describe("Moving cursor to top left");
    dispatch!(async; instance, MoveCursorToCorner, Corner::TopLeft).await?;

    describe("Moving cursor to top right");
    dispatch!(async; instance, MoveCursorToCorner, Corner::TopRight).await?;

    describe("Moving cursor to bottom right");
    dispatch!(async; instance, MoveCursorToCorner, Corner::BottomRight).await?;

    describe("Moving cursor to bottom left");
    dispatch!(async; instance, MoveCursorToCorner, Corner::BottomLeft).await?;

    describe("Moving window to next workspace");
    dispatch!(async; instance, MoveToWorkspace, WorkspaceIdentifierWithSpecial::Relative(1), None)
        .await?;

    describe("Moving window to previous workspace");
    dispatch!(async; instance, MoveToWorkspace, WorkspaceIdentifierWithSpecial::Relative(-1), None)
        .await?;

    describe("Toggling fullscreen");
    dispatch!(async; instance, ToggleFullscreen, FullscreenType::Maximize).await?;
    describe("Reverting fullscreen");
    dispatch!(async; instance, ToggleFullscreen, FullscreenType::Maximize).await?;

    describe("Toggling floating window");
    dispatch!(async; instance, ToggleFloating, None).await?;
    describe("Reverting floating window");
    Dispatch::call_async(instance, ToggleFloating(None)).await?;

    describe("Toggling split layout");
    Dispatch::call_async(instance, ToggleSplit).await?;
    describe("Reverting split layout");
    Dispatch::call_async(instance, ToggleSplit).await?;

    describe("Toggling opaque");
    Dispatch::call_async(instance, ToggleOpaque).await?;
    describe("Reverting opaque");
    Dispatch::call_async(instance, ToggleOpaque).await?;

    describe("Closing window");
    Dispatch::call_async(instance, KillActiveWindow).await?;

    Ok(())
}
