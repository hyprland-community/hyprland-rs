use hyprland::dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::instance::Instance;
use hyprland::keyword::Keyword;
/// Demonstates the use of Hyprland-rs for asyncronous creation key bindings
/// and using submaps
///
/// Usage: cargo run --example bind
use std::io::Read;

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
    let instance = Instance::from_current_env()?;

    Keyword::set_async(&instance, "submap", "example").await?;
    hyprland::bind!(async; &instance, SUPER, Key, "I" => ToggleFloating, None).await?;
    hyprland::bind!(async; &instance, l | CTRL ALT, Key, "Delete" => Exec, "sudo reboot").await?; // Reboot including from lock screen
    hyprland::bind!(async; &instance, e | SUPER, Key, "C" => KillActiveWindow).await?; // Kill all your windows
    Keyword::set_async(&instance, "submap", "reset").await?;

    dispatch!(async; &instance, Custom, "submap", "example").await?;
    println!("Press enter to revert to default keymap");
    let _ = std::io::stdin().read(&mut [0u8])?;
    dispatch!(async; &instance, Custom, "submap", "reset").await?;
    Ok(())
}
