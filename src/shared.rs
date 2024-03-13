//! # The Shared Module
//!
//! This module provides shared private and public functions, structs, enum, and types
pub use async_trait::async_trait;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::env::{var, VarError};
use std::hash::{Hash, Hasher};
use std::{error, fmt, io};

#[derive(Debug)]
/// Error that unifies different error types used by Hyprland-rs
pub enum HyprError {
    /// Error coming from serde
    SerdeError(serde_json::Error),
    /// Error coming from std::io
    IoError(io::Error),
    /// Error that occurs when parsing UTF-8 string
    FromUtf8Error(std::string::FromUtf8Error),
    /// Dispatcher returned non `ok` value
    NotOkDispatch(String),
}

impl From<io::Error> for HyprError {
    fn from(error: io::Error) -> Self {
        HyprError::IoError(error)
    }
}

impl From<serde_json::Error> for HyprError {
    fn from(error: serde_json::Error) -> Self {
        HyprError::SerdeError(error)
    }
}

impl From<std::string::FromUtf8Error> for HyprError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        HyprError::FromUtf8Error(error)
    }
}

impl fmt::Display for HyprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IoError(err) => err.to_string(),
                Self::SerdeError(err) => err.to_string(),
                Self::FromUtf8Error(err) => err.to_string(),
                Self::NotOkDispatch(msg) => format!(
                    "A dispatcher retrurned a non `ok`, value which is probably a error: {msg} was returned by it"
                ),
            }
        )
    }
}

impl error::Error for HyprError {}

/// This type provides the result type used everywhere in Hyprland-rs
#[deprecated(since = "0.3.1", note = "New location: hyprland::Result")]
pub type HResult<T> = Result<T, HyprError>;

/// The address struct holds a address as a tuple with a single value
/// and has methods to reveal the address in different data formats
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Address(String);

/// This trait provides a standardized way to get data
#[async_trait]
pub trait HyprData {
    /// This method gets the data
    fn get() -> crate::Result<Self>
    where
        Self: Sized;
    /// This method gets the data (async)
    async fn get_async() -> crate::Result<Self>
    where
        Self: Sized;
}

/// Trait for helper functions to get the active of the implementor
#[async_trait]
pub trait HyprDataActive {
    /// This method gets the active data
    fn get_active() -> crate::Result<Self>
    where
        Self: Sized;
    /// This method gets the active data (async)
    async fn get_active_async() -> crate::Result<Self>
    where
        Self: Sized;
}

/// Trait for helper functions to get the active of the implementor, but for optional ones
#[async_trait]
pub trait HyprDataActiveOptional {
    /// This method gets the active data
    fn get_active() -> crate::Result<Option<Self>>
    where
        Self: Sized;
    /// This method gets the active data (async)
    async fn get_active_async() -> crate::Result<Option<Self>>
    where
        Self: Sized;
}

/// This trait provides a standardized way to get data in a from of a vector
pub trait HyprDataVec<T>: HyprData {
    /// This method returns a vector of data
    fn to_vec(self) -> Vec<T>;
}

/// This type provides the id used to identify workspaces
/// > its a type because it might change at some point
pub type WorkspaceId = i32;

/// This type provides the id used to identify monitors
/// > its a type because it might change at some point
pub type MonitorId = i128;

fn ser_spec_opt(opt: &Option<String>) -> String {
    match opt {
        Some(name) => format!("special:{name}"),
        None => "special".to_string(),
    }
}

/// This enum holds workspace data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(untagged)]
pub enum WorkspaceType {
    /// A named workspace
    #[display(fmt = "{}", "_0")]
    Regular(
        /// The name
        String,
    ),
    /// The special workspace
    #[display(fmt = "{}", "ser_spec_opt(_0)")]
    Special(
        /// The name, if exists
        Option<String>,
    ),
}

impl From<&WorkspaceType> for String {
    fn from(value: &WorkspaceType) -> Self {
        value.to_string()
    }
}

// impl From<i8> for WorkspaceType {
//     fn from(int: i8) -> Self {
//         match int {
//             1.. => WorkspaceType::Unnamed(match int.try_into() {
//                 Ok(num) => num,
//                 Err(e) => panic!("Issue with parsing id (i8) as u8: {e}"),
//             }),
//             _ => panic!("Unrecognised id"),
//         }
//     }
// }

impl From<i32> for WorkspaceType {
    fn from(int: i32) -> Self {
        match int {
            1.. => WorkspaceType::Regular(int.to_string()),
            _ => panic!("Unrecognised id"),
        }
    }
}

impl Hash for WorkspaceType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            WorkspaceType::Regular(name) => name.hash(state),
            WorkspaceType::Special(value) => match value {
                Some(name) => name.hash(state),
                None => "".hash(state),
            },
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Address {
    /// This method returns a vector of bytes
    pub fn as_vec(self) -> Vec<u8> {
        let Address(value) = self;
        match hex::decode(value.trim_start_matches("0x")) {
            Ok(value) => value,
            Err(error) => panic!("A error has occured while parsing string as hex: {error}"),
        }
    }
    /// This creates a new address from a value that implements [std::string::ToString]
    pub fn new<T: ToString>(string: T) -> Self {
        Self(string.to_string())
    }
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) async fn write_to_socket(
    path: String,
    content: CommandContent,
) -> crate::Result<String> {
    use crate::unix_async::*;

    let mut stream = UnixStream::connect(path).await?;

    stream.write_all(&content.as_bytes()).await?;

    let mut response = vec![];

    const BUF_SIZE: usize = 8192;
    let mut buf = [0; BUF_SIZE];
    loop {
        let num_read = stream.read(&mut buf).await?;
        let buf = &buf[..num_read];
        response.append(&mut buf.to_vec());
        if num_read == 0 || num_read != BUF_SIZE {
            break;
        }
    }

    Ok(String::from_utf8(response)?)
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) fn write_to_socket_sync(path: String, content: CommandContent) -> crate::Result<String> {
    use io::prelude::*;
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(path)?;

    stream.write_all(&content.as_bytes())?;

    let mut response = vec![];

    const BUF_SIZE: usize = 8192;
    let mut buf = [0; BUF_SIZE];
    loop {
        let num_read = stream.read(&mut buf)?;
        let buf = &buf[..num_read];
        response.append(&mut buf.to_vec());
        if num_read == 0 || num_read != BUF_SIZE {
            break;
        }
    }

    Ok(String::from_utf8(response)?)
}

/// This pub(crate) enum holds the different sockets that Hyprland has
pub(crate) enum SocketType {
    /// The socket used to send commands to Hyprland (AKA `.socket.sock`)
    Command,
    /// The socket used to listen for events (AKA `.socket2.sock`)
    Listener,
}
/// This pub(crate) function gets the Hyprland socket path
pub(crate) fn get_socket_path(socket_type: SocketType) -> String {
    let hypr_instance_sig = match var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(var) => var,
        Err(VarError::NotPresent) => panic!("Is hyprland running?"),
        Err(VarError::NotUnicode(_)) => panic!("wtf no unicode?"),
    };

    let socket_name = match socket_type {
        SocketType::Command => ".socket.sock",
        SocketType::Listener => ".socket2.sock",
    };

    format!("/tmp/hypr/{hypr_instance_sig}/{socket_name}")
}

/// Creates a `CommandContent` instance with the given flag and formatted data.
///
/// # Arguments
///
/// * `$flag` - A `CommandFlag` variant (`JSON` or `Empty`) that represents the flag for the command.
/// * `$($k:tt)*` - A format string and its arguments to be used as the data in the `CommandContent` instance.
#[macro_export]
macro_rules! command {
    ($flag:ident, $($k:tt)*) => {{
        CommandContent {
            flag: CommandFlag::$flag,
            data: format!($($k)*),
        }
    }};
}
pub use command;

/// This enum defines the possible command flags that can be used.
pub enum CommandFlag {
    /// The JSON flag.
    JSON,
    /// An empty flag.
    Empty,
}

/// This struct defines the content of a command, which consists of a flag and a data string.
pub struct CommandContent {
    /// The flag for the command.
    pub flag: CommandFlag,
    /// The data string for the command.
    pub data: String,
}

impl CommandContent {
    /// Converts the command content to a byte vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyprland::shared::*;
    ///
    /// let content = CommandContent { flag: CommandFlag::JSON, data: "foo".to_string() };
    /// let bytes = content.as_bytes();
    /// assert_eq!(bytes, b"j/foo");
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl fmt::Display for CommandContent {
    /// Formats the command content as a string for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyprland::shared::*;
    ///
    /// let content = CommandContent { flag: CommandFlag::JSON, data: "foo".to_string() };
    /// let s = format!("{}", content);
    /// assert_eq!(s, "j/foo");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.flag {
            CommandFlag::JSON => write!(f, "j/{}", &self.data),
            CommandFlag::Empty => write!(f, "/{}", &self.data),
        }
    }
}
