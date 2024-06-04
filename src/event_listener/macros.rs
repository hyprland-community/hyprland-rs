macro_rules! add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name $end,$f,$c,$c2 => $id);
        add_async_listener!($name $end,$f,$c,$c2 => $id);
    };
    ($name:ident $end:ident,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name $end,$c,$c2 => $id);
        add_async_listener!($name $end,$c,$c2 => $id);
    };
    ($name:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$f,$c,$c2 => $id);
        add_async_listener!($name,$f,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$c,$c2 => $id);
        add_async_listener!($name,$c,$c2 => $id);
    };
}

macro_rules! add_listener_reg_raw {
    ($name:ident,$list_name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListener {
                #[doc = concat!("This method adds an event which executes when", stringify!($c), r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: $f) {
                    self.events.[<$list_name _events>].push(Box::new(f));
                }
            }
        }
    };
}

macro_rules! add_listener_reg {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            add_listener_reg_raw!($name,[<$name $end>],impl Fn($f) + 'static,$c,$c2 => $id);
        }
    };
    ($name:ident $end:ident,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            add_listener_reg_raw!($name,[<$name $end>],impl Fn() + 'static,$c,$c2 => $id);
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        add_listener_reg_raw!($name,$name,impl Fn($f) + 'static,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:expr => $id:ident) => {
        add_listener_reg_raw!($name,$name,impl Fn() + 'static,$c,$c2 => $id);
    };
}

macro_rules! add_async_listener_raw {
    ($name:ident,$list_name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncEventListener {
                #[doc = concat!("This method adds an event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#)]
                pub fn [<add_ $name _handler>](&mut self, f: $f) {
                    self.events.[<$list_name _events>].push(Box::pin(f));
                }
            }
        }
    };
}

macro_rules! add_async_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            add_async_listener_raw!($name,[<$name $end>],impl Fn($f) -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
        }
    };
    ($name:ident $end:ident,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            add_async_listener_raw!($name,[<$name $end>],impl Fn() -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        add_async_listener_raw!($name,$name,impl Fn($f) -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:expr => $id:ident) => {
        add_async_listener_raw!($name,$name,impl Fn() -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
    };
}

macro_rules! arm {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure(item, $val.clone());
        }
    }};
    ($nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_empty_closure(item);
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
    ($nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_empty_closure_async(item).await;
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
            special_removed_events: vec![],
            special_changed_events: vec![],
            keyboard_layout_change_events: vec![],
            sub_map_changed_events: vec![],
            layer_open_events: vec![],
            layer_closed_events: vec![],
            float_state_events: vec![],
            urgent_state_events: vec![],
            minimize_events: vec![],
            window_title_changed_events: vec![],
            screencast_events: vec![],
            config_reloaded_events: vec![],
        }
    };
}
