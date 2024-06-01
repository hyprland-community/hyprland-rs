/// Demonstates the use of Hyprland-rs for asyncronous creation key bindings
/// and using submaps
/// 
/// Usage: cargo run --example bind

use std::io::Read;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::keyword::Keyword;

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
    Keyword::set_async("submap", "example").await?;
    hyprland::bind!(async; SUPER, Key, "I" => ToggleFloating, None).await?;   
    hyprland::bind!(async; l | CTRL ALT, Key, "Delete" => Exec, "sudo reboot").await?; // Reboot including from lock screen
    hyprland::bind!(async; e | SUPER, Key, "C" => KillActiveWindow).await?; // Kill all your windows  
    Keyword::set_async("submap", "reset").await?;

    dispatch!(async; Custom, "submap", "example").await?;
    println!("Press enter to revert to default keymap");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    dispatch!(async; Custom, "submap", "reset").await?;
    Ok(())
}