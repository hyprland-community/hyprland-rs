// macro_rules! gen_event_adder {
//     ($name:literal,$str:tt) => {
//         format!("listener.{}(|data| println!({}))", $name, $str)
//     };
// }

macro_rules! add_listener {
    (reg $name:ident,$field:ident,$f:ty,$c:literal,$c2:expr) => {
        #[doc = $c]
        ///
        ///
        ///```rust, no_run
        ///use hyprland::event_listener::EventListener;
        ///let mut listener = EventListener::new();
        #[doc = $c2]
        ///listener.start_listener();
        ///```
        pub fn $name(&mut self, f: impl Fn($f) + 'static) {
            self.events.$field.push(EventTypes::Regular(Box::new(f)));
        }
    };
    ($name:ident,$field:ident,$f:ty) => {
        pub fn $name(&mut self. f: impl Fn($F) + 'static) {
            self.events.$field.push(EventTypes::Regular(Box::new(f)));
        }
    };

}

macro_rules! mut_add_listener {
    (reg $name:ident,$field:ident,$f:ty,$c:literal,$c2:expr) => {
        #[doc = $c]
        ///
        ///
        ///```rust, no_run
        ///use hyprland::event_listener::EventListenerMutable as EventListener;
        ///let mut listener = EventListener::new();
        #[doc = $c2]
        ///listener.start_listener();
        ///```
        pub fn $name(&mut self, f: impl Fn($f, &mut State) + 'static) {
            self.events
                .$field
                .push(EventTypes::MutableState(Box::new(f)));
        }
    };
    ($name:ident,$field:ident,$f:ty) => {
        pub fn $name(&mut self, f: impl Fn($f, &mut State) + 'static) {
            self.events
                .$field
                .push(EventTypes::MutableState(Box::new(f)));
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

macro_rules! arm_sync {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure(item, $val);
        }
    }};
}

// macro_rules! arm_async {
//     ($val:expr,$nam:ident,$se:ident) => {{
//         let events = &$se.events.$nam;
//         for item in events.iter() {
//             execute_closure(item, $val).await;
//         }
//     }};
// }
