use super::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::instance::AsyncInstance;
use futures_lite::{Stream, StreamExt};

/// Event listener, but [Stream]
/// This is the new prefered way of listening for events
/// as its more idiomatic, and allows for more efficient memory management
///
/// # Examples
/// ```rust, no_run
/// use hyprland::prelude::*;
/// use hyprland::event_listener::EventStream;
/// use hyprland::Result as HResult;
///
/// #[tokio::main]
/// async fn main() -> HResult<()> {
///     use futures_lite::StreamExt;
///     use hyprland::instance::AsyncInstance;
///     let instance = AsyncInstance::from_current_env()?;
///     let mut stream = EventStream::new(instance);
///     while let Some(Ok(event)) = stream.next().await {
///          println!("{event:?}");
///     }
/// }
/// ```
#[must_use = "streams nothing unless polled"]
pub struct EventStream {
    stream: Pin<Box<dyn Stream<Item = crate::Result<Event>> + Send>>,
}
impl EventStream {
    /// Creates a new [EventStream]
    pub fn new(mut instance: AsyncInstance) -> Self {
        use crate::async_import::*;
        let stream = async_stream::try_stream! {
        let stream: &mut UnixStream = instance.get_event_stream()?;
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
                for primed_event in event_primer_noexec(event, &mut active_windows)? {
                    yield primed_event;
                }
            }
        }
        };
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl Stream for EventStream {
    type Item = crate::Result<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().stream.poll_next(cx)
    }
}
