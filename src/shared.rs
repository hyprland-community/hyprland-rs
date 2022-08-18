//! # The Shared Module
//!
//! This module provides shared private and public functions, structs, enum, and types
use serde::{Deserialize, Deserializer, Serialize};
use std::env::{var, VarError};
use std::{fmt, io};

/// The address struct holds a address as a tuple with a single value
/// and has methods to reveal the address in different data formats
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Address(String);

/// This type provides the id used to identify workspaces
/// > its a type because it might change at some point
pub type WorkspaceId = u8;

/// This enum holds workspace data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum WorkspaceType {
    /// A regular workspace
    Regular(
        /// The workspace id
        WorkspaceId,
    ),
    /// The special workspace
    Special,
}

impl From<i8> for WorkspaceType {
    fn from(int: i8) -> Self {
        match int {
            -99 => WorkspaceType::Special,
            0.. => WorkspaceType::Regular(match int.try_into() {
                Ok(num) => num,
                Err(e) => panic!("Issue with parsing id (i8) as u8: {e}"),
            }),
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
            Err(error) => panic!("A error has occured while parsing string as hex: {}", error),
        }
    }
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) async fn write_to_socket(path: String, content: &[u8]) -> io::Result<String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::UnixStream;
    let mut stream = UnixStream::connect(path).await?;

    stream.write_all(content).await?;
    let mut response = [0; 8192];
    let num_read = stream.read(&mut response).await?;
    let response = &response[..num_read];
    Ok(match String::from_utf8(response.to_vec()) {
        Ok(str) => str,
        Err(error) => panic!("an error has occured while parsing bytes as utf8: {error:#?}"),
    })
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) fn write_to_socket_sync(path: String, content: &[u8]) -> io::Result<String> {
    use io::prelude::*;
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(path)?;

    stream.write_all(content)?;
    let mut response = [0; 8192];
    let num_read = stream.read(&mut response)?;
    let response = &response[..num_read];
    Ok(match String::from_utf8(response.to_vec()) {
        Ok(str) => str,
        Err(error) => panic!("an error has occured while parsing bytes as utf8: {error:#?}"),
    })
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

pub(crate) fn de_work_id<'de, D>(deserializer: D) -> Result<WorkspaceType, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux {
        Reg(u8),
        Special(i8),
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::Special(_) => Ok(WorkspaceType::Special),
        Aux::Reg(int) => Ok(WorkspaceType::Regular(int)),
    }
}
