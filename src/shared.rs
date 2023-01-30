//! # The Shared Module
//!
//! This module provides shared private and public functions, structs, enum, and types
pub use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize};
use std::env::{var, VarError};
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
//pub type HResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type HResult<T> = Result<T, HyprError>;

/// The address struct holds a address as a tuple with a single value
/// and has methods to reveal the address in different data formats
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Address(String);

/// This trait provides a standardized way to get data
#[async_trait]
pub trait HyprData {
    /// This method gets the data
    fn get() -> HResult<Self>
    where
        Self: Sized;
    /// This method gets the data (async)
    async fn get_async() -> HResult<Self>
    where
        Self: Sized;
}

/// Trait for helper functions to get the active of the implementor
#[async_trait]
pub trait HyprDataActive {
    /// This method gets the active data
    fn get_active() -> HResult<Self>
    where
        Self: Sized;
    /// This method gets the active data (async)
    async fn get_active_async() -> HResult<Self>
    where
        Self: Sized;
}

/// Trait for helper functions to get the active of the implementor, but for optional ones
#[async_trait]
pub trait HyprDataActiveOptional {
    /// This method gets the active data
    fn get_active() -> HResult<Option<Self>>
    where
        Self: Sized;
    /// This method gets the active data (async)
    async fn get_active_async() -> HResult<Option<Self>>
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

/// This enum holds workspace data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum WorkspaceType {
    /// A named workspace
    Regular(
        /// The name
        String,
    ),
    /// The special workspace
    Special(
        /// The name, if exists
        Option<String>,
    ),
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
pub(crate) async fn write_to_socket(path: String, content: &[u8]) -> HResult<String> {
    use crate::unix_async::*;

    let mut stream = UnixStream::connect(path).await?;

    stream.write_all(content).await?;
    let mut response = [0; 8192];
    let num_read = stream.read(&mut response).await?;
    let response = &response[..num_read];
    Ok(String::from_utf8(response.to_vec())?)
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) fn write_to_socket_sync(path: String, content: &[u8]) -> HResult<String> {
    use io::prelude::*;
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(path)?;

    stream.write_all(content)?;
    let mut response = [0; 8192];
    let num_read = stream.read(&mut response)?;
    let response = &response[..num_read];
    Ok(String::from_utf8(response.to_vec())?)
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

pub(crate) fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        T(T),
        Empty(Empty),
        Null,
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}
