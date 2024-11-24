use super::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_stream::try_stream;
use futures_lite::{Stream, StreamExt};

/// Event listener, but [Stream]
/// This is the new prefered way of listening for events
/// as its more idiomatic, and allows for more efficient memory management
///
/// # Examples
/// ```rust
/// use hyprland::prelude::*;
/// use hyprland::event_listener::EventStream;
/// use hyprland::Result as HResult;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> HResult<()> {
///     let mut stream = EventStream::new();
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
    pub fn new() -> Self {
        use crate::unix_async::*;
        let stream = try_stream! {

        let socket_path = get_socket_path(SocketType::Listener)?;
        let mut stream = UnixStream::connect(socket_path).await?;

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
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().stream.poll_next(cx)
    }

    type Item = crate::Result<Event>;
}
