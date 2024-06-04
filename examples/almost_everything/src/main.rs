use std::fmt::Display;

use hyprland::data::{Client, Clients, Monitors, Workspace};
use hyprland::dispatch::*;
use hyprland::event_listener::EventListener;
use hyprland::keyword::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;

fn main() -> hyprland::Result<()> {
    // We can call dispatchers with the dispatch macro, and struct!
    // You can decide what you want to use, below are some examples of their usage

    // Here we are telling hyprland to open kitty using the dispatch macro!
    hyprland::dispatch!(Exec, "kitty")?;

    // Here we are moving the cursor to the top left corner! We can also just use the Dispatch
    // struct!
    Dispatch::call(DispatchType::MoveCursorToCorner(Corner::TopLeft))?;

    // Here we are adding a keybinding to Hyprland using the bind macro!
    hyprland::bind!(SUPER, Key, "i" => ToggleFloating, None)?;

    // Here we are getting the border size
    let border_size = match Keyword::get("general:border_size")?.value {
        OptionValue::Int(i) => i,
        _ => panic!("border size can only be a int"),
    };
    println!("{border_size}");

    // Here we change a keyword, in this case we are doubling the border size we got above
    Keyword::set("general:border_size", border_size * 2)?;

    // get all monitors
    let monitors = Monitors::get()?;

    // and the active window
    let win = Client::get_active()?;

    // and all open windows
    let clients = Clients::get()?;

    // and the active workspace
    let work = Workspace::get_active()?;

    // and printing them all out!
    println!("monitors: {monitors:#?},\nactive window: {win:#?},\nclients {clients:#?}\nworkspace: {work:#?}");

    // Create a event listener
    let mut event_listener = EventListener::new()?;

    // Shows when active window changes
    event_listener.add_active_window_change_handler(|data| {
        println!("{data:#?}");
    });

    // This changes the workspace to 5 if the workspace is switched to 9
    // this is a performance and mutable state test
    event_listener.add_workspace_change_handler(|id, state| {
        if id == WorkspaceType::Regular('9'.to_string()) {
            state.active_workspace = WorkspaceType::Regular('2'.to_string());
        }
    });
    // This makes it so you can't turn on fullscreen lol
    event_listener.add_fullscreen_state_change_handler(|fstate| {
        if fstate {
            dispatch!(ToggleFullscreen, FullscreenType::Real);
        }
    });
    // Makes a monitor unfocusable
    event_listener.add_active_monitor_change_handler(|data, state| {
        let hyprland::event_listener::MonitorEventData { monitor_name, .. } = data;

        if monitor_name == *"DP-1".to_string() {
            state.active_monitor = "eDP-1".to_string()
        }
    });

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(|id| println!("workspace changed to {id:#?}"));

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener()
}
