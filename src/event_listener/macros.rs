// macro_rules! gen_event_adder {
//     ($name:literal,$str:tt) => {
//         format!("listener.{}(|data| println!({}))", $name, $str)
//     };
// }

// #[macro_export]
// macro_rules! asyncfn {
//     ($code:expr) => {
//         Box::pin(async move {
//             {
//                 $code
//             }
//         })
//     };
// }

macro_rules! add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name $end,$f,$c,$c2 => $id);
        mut_add_listener!($name $end,$f,$c,$c2 => $id);
        add_async_listener!($name $end,$f,$c,$c2 => $id);
        //add_mut_async_listener!($name $end,$f,$c,$c2 => $id);
    };
    ($name:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$f,$c,$c2 => $id);
        mut_add_listener!($name,$f,$c,$c2 => $id);
        add_async_listener!($name,$f,$c,$c2 => $id);
        //add_mut_async_listener!($name,$f,$c,$c2 => $id);
    };
}

macro_rules! add_listener_reg {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListener {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name $end _events>].push(EventTypes::Regular(Box::new(f)));
                }
            }
        }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListener {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name _events>].push(EventTypes::Regular(Box::new(f)));
                }
            }
        }
        }
    };
}

macro_rules! add_async_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncEventListener {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name $end _events>].push(AsyncEventTypes::Regular(Box::pin(f)));
                }
            }
        }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncEventListener {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name _events>].push(AsyncEventTypes::Regular(Box::pin(f)));
                }
            }
        }
        }
    };
}

#[allow(unused_macros)]
macro_rules! add_mut_async_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncMutableEventListener {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut StateV2) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name $end _events>].push(AsyncEventTypes::MutableState(Box::pin(f)));
                }
            }
        }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl AsyncMutableEventListener {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut StateV2) -> VoidFuture + Send + Sync + 'static) {
                    self.events.[<$name _events>].push(AsyncEventTypes::MutableState(Box::pin(f)));
                }
            }
        }
        }
    };
}

macro_rules! mut_add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListenerMutable {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListenerMutable as EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#", _| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut State) + 'static) {
                    self.events.[<$name $end _events>].push(EventTypes::MutableState(Box::new(f)));
                }
            }
        }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            impl EventListenerMutable {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListenerMutable as EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#", _| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut State) + 'static) {
                    self.events.[<$name _events>].push(EventTypes::MutableState(Box::new(f)));
                }
            }
        }
        }
    };
}

macro_rules! mut_arm {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for i in events.iter() {
            let new_state = execute_closure_mut($se.state.clone(), i, $val).await?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_state_arm {
    ($val:expr,$nam:ident,$na:ident,$va:expr,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na = $va;
        for i in events.iter() {
            let new_state = execute_closure_mut($se.state.clone(), i, $val).await?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_arm_sync {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for i in events.iter() {
            let new_state = execute_closure_mut_sync($se.state.clone(), i, $val)?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_state_arm_sync {
    ($val:expr,$nam:ident,$na:ident,$va:expr,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na = $va;
        for i in events.iter() {
            let new_state = execute_closure_mut_sync($se.state.clone(), i, $val)?;
            $se.state = new_state;
        }
    }};
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
            execute_closure(item, $val);
        }
    }};
}

macro_rules! arm_async {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure_async(item, $val).await;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! arm_async_mut {
    (c $val:expr,$nam:ident,$na:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na.update({ $val }.into());
        for item in events.iter() {
            execute_closure_async_state(item, $val, &mut $se.state).await;
        }
    }};
    (cv $val:expr,$nam:ident,$na:ident,$va:expr,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na.update({ $va }.clone());
        for item in events.iter() {
            execute_closure_async_state(item, $val, &mut $se.state).await;
        }
    }};
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure_async_state(item, $val, &mut $se.state).await;
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
            screencopy_events: vec![],
        }
    };
}
