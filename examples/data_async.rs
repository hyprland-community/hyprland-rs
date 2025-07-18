/// Demostrates asynchronously using Hyprland-rs to fetch information about your Hyprland environment
///
/// Usage: cargo run --example data_async <animations|binds|client(s)|workspace(s)|monitor(s)>
/// Example: cargo run --example data_async client        (Gets data on active client)
/// Example: cargo run --example data_async workspaces    (Gets data on all workspaces)
use hyprland::data::{
    Animations, Binds, Client, Clients, Monitor, Monitors, Workspace, Workspaces,
};
use hyprland::instance::AsyncInstance;
use hyprland::shared::{HyprData, HyprDataActive, HyprDataActiveOptional};


#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        panic!("You have to specify client, workspace or monitor")
    }

    let mut instance = AsyncInstance::from_current_env().await?;
    match args[0].as_str() {
        "client" => println!("{:#?}", Client::get_active_async(&mut instance).await?),
        "monitor" => println!("{:#?}", Monitor::get_active_async(&mut instance).await?),
        "workspace" => println!("{:#?}", Workspace::get_active_async(&mut instance).await?),
        "animations" => println!("{:#?}", Animations::get_async(&mut instance).await?),
        "binds" => println!("{:#?}", Binds::get_async(&mut instance).await?),
        "clients" => println!("{:#?}", Clients::get_async(&mut instance).await?),
        "monitors" => println!("{:#?}", Monitors::get_async(&mut instance).await?),
        "workspaces" => println!("{:#?}", Workspaces::get_async(&mut instance).await?),
        _ => println!("Specify one of client(s), monitor(s) or workspace(s)"),
    };

    Ok(())
}
