/// Demostrats using hyprland-rs to asynchronously listen for events
/// 
/// Usage: cargo run --example events

use hyprland::async_closure;
use hyprland::event_listener::AsyncEventListener;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> hyprland::Result<()> {
        // Create a event listener
        let mut event_listener = AsyncEventListener::new();

        event_listener.add_active_window_change_handler(async_closure! {
            |data| println!("{data:#?}")
        });
    
        event_listener.add_fullscreen_state_change_handler(async_closure! {
            |fstate| println!("Window {} fullscreen", if fstate { "is" } else { "is not" })
        });

        event_listener.add_active_monitor_change_handler(async_closure! {
            |state| println!("Monitor state: {state:#?}")
        });
    
        // add event, yes functions and closures both work!
        event_listener.add_workspace_change_handler(async_closure! {
            |id| println!("workspace changed to {id:?}")
        });
    
        // and execute the function
        // here we are using the blocking variant
        // but there is a async version too
        event_listener.start_listener_async().await?;

        return Ok(());
}