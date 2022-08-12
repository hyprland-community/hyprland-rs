use hyprland::data::blocking::get_monitors;
use hyprland::dispatch::{dispatch_blocking, Corner, DispatchType};
use hyprland::event_listener::EventListenerMutable as EventListener;

fn main() -> std::io::Result<()> {
    // We can call dispatchers with the dispatch function!

    // Here we are telling hyprland to open kitty!
    dispatch_blocking(DispatchType::Exec("kitty".to_string()))?;

    // Here we are moving the cursor to the top left corner!
    dispatch_blocking(DispatchType::MoveCursorToCorner(Corner::TopLeft))?;

    // Here we change a keyword, yes its a dispatcher don't complain
    dispatch_blocking(DispatchType::Keyword(
        "general:border_size".to_string(),
        "30".to_string(),
    ))?;

    // get all monitors
    let monitors = get_monitors();

    // and printing them all out!
    println!("{monitors:#?}");

    // Create a event listener
    let mut event_listener = EventListener::new();

    event_listener.add_workspace_change_handler(|id, state| {
        if id == 2 {
            state.active_workspace = 5;
        }
    });
    event_listener.add_fullscreen_state_change_handler(|fstate, state| {
        if fstate {
            state.fullscreen_state = false;
        }
    });

    event_listener.add_active_monitor_change_handler(|data, state| {
        let hyprland::event_listener::MonitorEventData(monitor, _) = data;

        if monitor == *"DP-1".to_string() {
            state.active_monitor = "eDP-1".to_string()
        }
    });

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(|id, _| println!("workspace changed to {id:#?}"));

    // Waybar example
    // event_listener.add_active_window_change_handler(|data| {
    //     use hyprland::event_listener::WindowEventData;
    //     let string = match data {
    //         Some(WindowEventData(class, title)) => format!("{class}: {title}"),
    //         None => "".to_string()
    //     };
    //     println!(r#"{{"text": "{string}", class: "what is this?"}}"#);
    // });

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener_blocking()
}
