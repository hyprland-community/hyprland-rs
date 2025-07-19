use hyprland::dispatch::DispatchType::*;
use hyprland::dispatch::{
    Corner, Dispatch, FullscreenType, WorkspaceIdentifierWithSpecial,
};
/// Demonstrates usage of various dispatch calls
///
/// Usage: cargo run --example dispatch <hyprland args>? <program_name>? <program_args>?
/// Example: cargo run --example dispatch [workspace 2] kitty
use hyprland::{default_instance_panic, dispatch};

fn describe(desc: &str) {
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{desc}");
}

fn main() -> hyprland::Result<()> {
    let program = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    let instance = default_instance_panic();

    dispatch!(instance, Exec, &program)?;

    describe("Moving cursor to top left");
    dispatch!(instance, MoveCursorToCorner, Corner::TopLeft)?;

    describe("Moving cursor to top right");
    dispatch!(instance, MoveCursorToCorner, Corner::TopRight)?;

    describe("Moving cursor to bottom right");
    dispatch!(instance, MoveCursorToCorner, Corner::BottomRight)?;

    describe("Moving cursor to bottom left");
    dispatch!(instance, MoveCursorToCorner, Corner::BottomLeft)?;

    describe("Moving window to next workspace");
    dispatch!(
        instance,
        MoveToWorkspace,
        WorkspaceIdentifierWithSpecial::Relative(1),
        None
    )?;

    describe("Moving window to previous workspace");
    dispatch!(
        instance,
        MoveToWorkspace,
        WorkspaceIdentifierWithSpecial::Relative(-1),
        None
    )?;

    describe("Toggling fullscreen");
    dispatch!(instance, ToggleFullscreen, FullscreenType::Maximize)?;
    describe("Reverting fullscreen");
    Dispatch::call(instance, ToggleFullscreen(FullscreenType::Maximize))?;

    describe("Toggling floating window");
    dispatch!(instance, ToggleFloating, None)?;
    describe("Reverting floating window");
    Dispatch::call(instance, ToggleFloating(None))?;

    describe("Toggling split layout");
    Dispatch::call(instance, ToggleSplit)?;
    describe("Reverting split layout");
    Dispatch::call(instance, ToggleSplit)?;

    describe("Toggling opaque");
    Dispatch::call(instance, ToggleOpaque)?;
    describe("Reverting opaque");
    Dispatch::call(instance, ToggleOpaque)?;

    describe("Closing window");
    Dispatch::call(instance, KillActiveWindow)?;

    Ok(())
}
