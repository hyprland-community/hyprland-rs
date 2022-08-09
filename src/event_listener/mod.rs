mod shared;
pub use crate::event_listener::shared::*;
/// placeholder
pub mod mutable;
pub use crate::event_listener::immutable::EventListener;

mod immutable;
pub use crate::event_listener::mutable::EventListener as EventListenerMutable;
