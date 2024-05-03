macro_rules! add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name $end,$f,$c,$c2 => $id);
        add_async_listener!($name $end,$f,$c,$c2 => $id);
    };
    ($name:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$f,$c,$c2 => $id);
        add_async_listener!($name,$f,$c,$c2 => $id);
    };
}

macro_rules! add_listener_reg {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListener {
                    #[doc = concat!("This methods adds a event which ", stringify!($c), r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name $end _events>].push(Box::new(f));
                }
            }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListener {
                #[doc = concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name _events>].push(Box::new(f));
                }
            }
        }
    };
}

macro_rules! add_async_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncEventListener {
                #[doc = concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name $end _events>].push(Box::pin(f));
                }
            }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncEventListener {
                #[doc = concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name _events>].push(Box::pin(f));
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! arm_alpha {
    ($sync:ident; $val:expr,$nam:ident,$se:ident) => {{
        paste! {
            let events = &$se.events.$nam;
            for item in events.iter() {
                [<execute_closure_ $sync>](item, $val);
            }
        }
    }};
}

macro_rules! arm {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure(item, $val.clone());
        }
    }};
}

macro_rules! arm_async {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure_async(item, $val.clone()).await;
        }
    }};
}

macro_rules! init_events {
    ($name:ident) => {
        $name {
            workspace_changed_events: vec![],
            workspace_added_events: vec![],
            workspace_destroyed_events: vec![],
            workspace_moved_events: vec![],
            workspace_rename_events: vec![],
            active_monitor_changed_events: vec![],
            active_window_changed_events: vec![],
            fullscreen_state_changed_events: vec![],
            monitor_removed_events: vec![],
            monitor_added_events: vec![],
            window_open_events: vec![],
            window_close_events: vec![],
            window_moved_events: vec![],
            keyboard_layout_change_events: vec![],
            sub_map_changed_events: vec![],
            layer_open_events: vec![],
            layer_closed_events: vec![],
            float_state_events: vec![],
            urgent_state_events: vec![],
            minimize_events: vec![],
            window_title_changed_events: vec![],
            screencast_events: vec![],
        }
    };
}
