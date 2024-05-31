/// Demonstates usage of dispatch to execute a program in Hyprland
/// 
/// Usage: cargo run --example exec -- <hyprland args>? <program_name> <program_args>?
/// Example: cargo run --example exec -- [workspace 2] kitty 
use hyprland::dispatch::*;

fn main() -> hyprland::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("Executing {:?}", args);
    hyprland::dispatch!(Exec, &args)?;
    return Ok(());
}