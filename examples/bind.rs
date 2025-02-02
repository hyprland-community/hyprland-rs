/// Demonstates the use of Hyprland-rs for creating key bindings
/// and using submaps
/// 
/// Usage: cargo run --example bind

use std::io::Read;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::keyword::Keyword;

fn main() -> hyprland::Result<()> {
    Keyword::set("submap", "example")?;
    hyprland::bind!(SUPER, Key, "I" => ToggleFloating, None)?;   
    hyprland::bind!(l | CTRL ALT, Key, "Delete" => Exec, "sudo reboot")?; // Reboot including from lock screen
    hyprland::bind!(e | SUPER, Key, "C" => KillActiveWindow)?; // Kill all your windows  
    Keyword::set("submap", "reset")?;

    dispatch!(Custom, "submap", "example")?;
    println!("Press enter to revert to default keymap");
    let _ = std::io::stdin().read(&mut [0u8])
        .expect("Crashed: Run `hyprctl dispatch submap reset` to return to default submap");
    dispatch!(Custom, "submap", "reset")?;
    
    Ok(())
}