macro_rules! events {
    ($($name:ty => $f:ty,$c:literal,$c2:literal => $id:ident);*) => {
        paste! {
            pub(crate) struct Events {
                $(
                    pub(crate) [<$name:snake _events>]: type_if! {(),$f,Vec<EmptyClosure>, Closures<$f>}
                ),*
            }
            #[allow(clippy::type_complexity)]
            pub(crate) struct AsyncEvents {
                $(
                    pub(crate) [<$name:snake _events>]: type_if! {(),$f,Vec<EmptyAsyncClosure>, AsyncClosures<$f>}
                ),*
            }
            pub(crate) fn create_events() -> Events {
                Events {
                    $([<$name:snake _events>]: vec![]),*
                }
            }
            pub(crate) fn create_events_async() -> AsyncEvents {
                AsyncEvents {
                    $([<$name:snake _events>]: vec![]),*
                }
            }

            impl HasAsyncExecutor for AsyncEventListener {
                async fn event_executor_async(&mut self, event: Event) -> crate::Result<()> {
                    use Event::*;
                    match event {
                        $(
                            expr_if! {(),$f, $name, $name($id)} => expr_if! {
                                (),
                                $f,
                                arm_async!([<$name:snake _events>], self),
                                arm_async!($id, [<$name:snake _events>], self)
                            },
                        )*
                        _ => ()
                    }
                    Ok(())
                }
            }
            impl HasExecutor for EventListener {
                fn event_executor(&mut self, event: Event) -> crate::Result<()> {
                    use Event::*;
                    match event {
                        $(
                            expr_if! {(),$f, $name, $name($id)} => expr_if! {
                                (),
                                $f,
                                arm!([<$name:snake _events>], self),
                                arm!($id, [<$name:snake _events>], self)
                            },
                        )*
                        _ => ()
                    }
                    Ok(())
                }
            }
        }
        $(
            paste!{
                block_if!{
                    (),
                    $f,
                    {
                        add_listener!{[<$name:snake>],$c,$c2 => $id}
                    },
                    {
                        add_listener!{[<$name:snake>],$f,$c,$c2 => $id}
                    }
                }
            }
        )*
    };
}

macro_rules! add_listener {
    ($name:ident,$f:ty,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$f,$c,$c2 => $id);
        add_async_listener!($name,$f,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:literal => $id:ident) => {
        add_listener_reg!($name,$c,$c2 => $id);
        add_async_listener!($name,$c,$c2 => $id);
    };
}

macro_rules! add_async_listener {
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        add_async_listener_raw!($name,$name,impl Fn($f) -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:expr => $id:ident) => {
        add_async_listener_raw!($name,$name,impl Fn() -> VoidFuture + Send + Sync + 'static,$c,$c2 => $id);
    };
}
macro_rules! add_listener_reg {
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        add_listener_reg_raw!($name,$name,impl Fn($f) + 'static,$c,$c2 => $id);
    };
    ($name:ident,$c:literal,$c2:expr => $id:ident) => {
        add_listener_reg_raw!($name,$name,impl Fn() + 'static,$c,$c2 => $id);
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
