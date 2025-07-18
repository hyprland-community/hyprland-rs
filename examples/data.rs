/// Demostrates using hyprland-rs to fetch information about clients, workspaces and monitors
///
/// Usage: cargo run --example data <animations|binds|client(s)|workspace(s)|monitor(s)>
/// Example: cargo run --example data client        (Gets data on active client)
/// Example: cargo run --example data workspaces    (Gets data on all workspaces)
use hyprland::data::{
    Animations, Binds, Client, Clients, Monitor, Monitors, Workspace, Workspaces,
};
use hyprland::instance::Instance;
use hyprland::shared::{HyprData, HyprDataActive, HyprDataActiveOptional};

fn main() -> hyprland::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() {
        panic!("You have to specify client, workspace or monitor")
    }

    let instance = Instance::from_current_env()?;
    match args[0].as_str() {
        "client" => println!("{:#?}", Client::get_active(&instance)?),
        "monitor" => println!("{:#?}", Monitor::get_active(&instance)?),
        "workspace" => println!("{:#?}", Workspace::get_active(&instance)?),
        "animations" => println!("{:#?}", Animations::get(&instance)?),
        "binds" => println!("{:#?}", Binds::get(&instance)?),
        "clients" => println!("{:#?}", Clients::get(&instance)?),
        "monitors" => println!("{:#?}", Monitors::get(&instance)?),
        "workspaces" => println!("{:#?}", Workspaces::get(&instance)?),
        _ => println!("Specify one of client(s), monitor(s) or workspace(s)"),
    };

    Ok(())
}
