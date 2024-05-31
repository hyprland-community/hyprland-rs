/// Demonstates asynchronous usage of dispatch to execute a program in Hyprland
/// 
/// Usage: cargo run --example exec_async -- <hyprland args>? <program_name> <program_args>?
/// Example: cargo run --example exec_async -- [workspace 2] kitty 
use hyprland::dispatch::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> hyprland::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("Executing {:?}", args);
    hyprland::dispatch!(async; Exec, &args).await?;
    return Ok(());
}