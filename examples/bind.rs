use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::instance::Instance;
use hyprland::keyword::Keyword;
use hyprland::dispatch;
/// Demonstates the use of Hyprland-rs for creating key bindings
/// and using submaps
///
/// Usage: cargo run --example bind
use std::io::Read;

fn main() -> hyprland::Result<()> {
    let instance = Instance::from_current_env()?;

    Keyword::set(&instance, "submap", "example")?;
    hyprland::bind!(&instance, SUPER, Key, "I" => ToggleFloating, None)?;
    hyprland::bind!(&instance, l | CTRL ALT, Key, "Delete" => Exec, "sudo reboot")?; // Reboot including from lock screen
    hyprland::bind!(&instance, e | SUPER, Key, "C" => KillActiveWindow)?; // Kill all your windows
    Keyword::set(&instance, "submap", "reset")?;

    Keyword::set(&instance, "submap", "example")?;
    hyprland::bind!(&instance, SUPER, Key, "I" => ToggleFloating, None)?;
    hyprland::bind!(&instance, l | CTRL ALT, Key, "Delete" => Exec, "sudo reboot")?; // Reboot including from lock screen
    hyprland::bind!(&instance, e | SUPER, Key, "C" => KillActiveWindow)?; // Kill all your windows
    Keyword::set(&instance, "submap", "reset")?;

    dispatch!(&instance, Custom, "submap", "example")?;
    println!("Press enter to revert to default keymap");
    let _ = std::io::stdin()
        .read(&mut [0u8])
        .expect("Crashed: Run `hyprctl dispatch submap reset` to return to default submap");
    dispatch!(&instance, Custom, "submap", "reset")?;

    Ok(())
}
