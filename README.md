# Hyprland-rs

[![Crates.io](https://img.shields.io/crates/v/hyprland)](https://crates.io/crates/hyprland)
![Crates.io](https://img.shields.io/crates/d/hyprland)
[![Crates.io](https://img.shields.io/crates/l/hyprland)](https://www.gnu.org/licenses/gpl-3.0.html)
[![docs.rs](https://img.shields.io/docsrs/hyprland)](https://docs.rs/hyprland)
[![Hyprland](https://img.shields.io/badge/Made%20for-Hyprland-blue)](https://github.com/hyprwm/Hyprland)
[![Discord](https://img.shields.io/discord/1055990214411169892?label=discord)](https://discord.gg/zzWqvcKRMy)

An unoffical rust wrapper for Hyprland's IPC

## Disclaimer
If something doesn't work, doesn't matter what,
make sure you are on the latest commit of Hyprland before making an issue!

## Getting started!

Lets get started with Hyprland-rs!

### Adding to your project

Add the code below to the dependencies section of your Cargo.toml file!

```toml
hyprland = "0.3.0"
```

### What this crate provides

This crate provides 3 modules (+1 for shared things)
 - `data` for getting information on the compositor
 - `event_listener` which provides the `EventListener` struct for listening for events
 - `dispatch` for calling dispatchers and changing keywords

## Example Usage

here is an example of most of the provided features being utilized

```rust ,no_run
use hyprland::data::Monitors;
use hyprland::keyword::*;
use hyprland::dispatch::{Dispatch, Corner, DispatchType};
use hyprland::event_listener::EventListener;
use hyprland::shared::HResult;
use hyprland::prelude::*;

fn main() -> HResult<()> {
    // We can call dispatchers with the dispatch function!

    // Here we are telling hyprland to open kitty!
    Dispatch::call(DispatchType::Exec("kitty".to_string()))?;

    // Here we are moving the cursor to the top left corner!
    Dispatch::call(DispatchType::MoveCursorToCorner(Corner::TopLeft))?;

    // Here we change a keyword, yes its a dispatcher don't complain
    Keyword::set(
        "general:border_size",
        30,
    )?;

    // get all monitors as a vector
    let monitors = Monitors::get()?.collect();

    // and printing them all out!
    println!("{monitors:#?}");

    // Create a event listener
    let mut event_listener = EventListener::new();

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(|id| println!("workspace changed to {id:#?}"));

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener()
}
```
