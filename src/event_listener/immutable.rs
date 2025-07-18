use super::*;
use crate::instance::{AsyncInstance, Instance};
use std::io;

/// This struct is used for adding event handlers and executing them on events
/// # The Event Listener
///
/// This struct holds what you need to create a event listener
///
/// ## Usage
///
/// ```rust, no_run
/// use hyprland::event_listener::EventListener;
/// let instance = hyprland::instance::Instance::from_current_env().unwrap();
/// let mut listener = EventListener::new(); // creates a new listener
/// // add a event handler which will be ran when this event happens
/// listener.add_workspace_changed_handler(|data| println!("{:#?}", data));
/// listener.start_listener(instance); // or `.start_listener_async().await` if async
/// ```
pub struct EventListener {
    pub(crate) events: Events,
}

impl Default for EventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl EventListener {
    /// This method creates a new EventListener instance
    ///
    /// ```rust
    /// use hyprland::event_listener::EventListener;
    /// let mut listener = EventListener::new();
    /// ```
    pub fn new() -> EventListener {
        EventListener {
            events: create_events(),
        }
    }

    /// This method starts the event listener (async)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// # async fn function() -> std::io::Result<()> {
    /// use hyprland::event_listener::EventListener;
    /// let instance = hyprland::instance::AsyncInstance::from_current_env().unwrap();
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_changed_handler(|id| println!("changed workspace to {id:?}"));
    /// listener.start_listener_async(instance).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_listener_async(&mut self, mut instance: AsyncInstance) -> crate::Result<()> {
        use crate::async_import::*;
        let stream = instance.get_event_stream()?;

        let mut active_windows = vec![];
        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf).await?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];
            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed {
                self.event_primer(event, &mut active_windows)?;
            }
        }

        Ok(())
    }

    /// This method starts the event listener (blocking)
    ///
    /// This should be ran after all of your handlers are defined
    /// ```rust, no_run
    /// use hyprland::event_listener::EventListener;
    /// let instance = hyprland::instance::Instance::from_current_env().unwrap();
    /// let mut listener = EventListener::new();
    /// listener.add_workspace_changed_handler(&|id| println!("changed workspace to {id:?}"));
    /// listener.start_listener(instance);
    /// ```
    pub fn start_listener(&mut self, mut instance: Instance) -> crate::Result<()> {
        use io::prelude::*;
        let stream = instance.get_event_stream()?;

        let mut active_windows = vec![];
        loop {
            let mut buf = [0; 4096];

            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }
            let buf = &buf[..num_read];
            let string = String::from_utf8(buf.to_vec())?;
            let parsed: Vec<Event> = event_parser(string)?;

            for event in parsed {
                self.event_primer(event, &mut active_windows)?;
            }
        }

        Ok(())
    }
}
