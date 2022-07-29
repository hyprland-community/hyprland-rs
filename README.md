# hyprland-rs
A unoffical rust wrapper for hyprland's IPC


## Example Code
```rust
use hyprland::data::get_monitors;
use hyprland::dispatch::{dispatch, DispatchType};
use hyprland::event_listener::EventListener;

fn main() -> std::io::Result<()> {
    // We can call dispatchers!
    // Here we are telling hyprland to open kitty!
    dispatch(DispatchType::Exec(String::from("kitty")))?;

    // get all monitors
    let monitors = get_monitors();

    // and printing them all out!
    println!("{monitors:#?}");

    // Create event listener
    let mut event_listener = EventListener::new();

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(&|id| println!("workspace changed to {id:#?}"));

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener_blocking()
}
```
