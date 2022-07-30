# Hyprland-rs
A unoffical rust wrapper for Hyprland's IPC

## Adding to Project
Right now the only way is to use a git submodule, this will be published to crates.to shortly!

## Example Code
Here is a example code snippet of examples of each function of this crate!
```rust
use hyprland::data::get_monitors;
use hyprland::dispatch::{dispatch, Corner, DispatchType};
use hyprland::event_listener::EventListener;

fn main() -> std::io::Result<()> {
    // We can call dispatchers with the dispatch function!

    // Here we are telling hyprland to open kitty!
    dispatch(DispatchType::Exec("kitty".to_string()))?;

    // Here we are moving the cursor to the top left corner!
    dispatch(DispatchType::MoveCursorToCorner(Corner::TopLeft))?;

    // Here we change a keyword, yes its a dispatcher don't complain
    dispatch(DispatchType::Keyword(
        "general:border_size".to_string(),
        "30".to_string(),
    ))?;

    // get all monitors
    let monitors = get_monitors();

    // and printing them all out!
    println!("{monitors:#?}");

    // Create a event listener
    let mut event_listener = EventListener::new();

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(&|id| println!("workspace changed to {id:#?}"));

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener_blocking()
}
```
