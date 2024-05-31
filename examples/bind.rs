/// Demonstates the use of Hyprland-rs for creating key bindings
/// 
/// Usage: cargo run --example bind

use std::io::Read;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType, FullscreenType};
use hyprland::keyword::Keyword;
//use hyprland::config::binds::Flag;

fn main() -> hyprland::Result<()> {

    //Keyword::set("submap", "example")?;
    hyprland::bind!(SUPER, Key, "i" => Exec, "kitty")?; 
    //bind!(r | SUPER, Key, "i" => ToggleFloating, None)?;

    dispatch!(Custom, "submap", "example")?;
    println!("Press enter to revert to default keymap");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    dispatch!(Custom, "submap", "reset");
    return Ok(());
}