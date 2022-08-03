//! # The Shared Module
//!
//! This module provides shared private and public functions, structs, enum, and types
use std::env::{var, VarError};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use serde::{Deserialize, Serialize};

/// The address struct holds a address as a tuple with a single value
/// and has methods to reveal the address in different data formats
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Address(String);

/// This type provides the id used to identify workspaces
/// > its a type because it might change at some point
pub type WorkspaceId = u8;

impl Address {
    /// This method returns a vector of bytes
    pub fn as_vec(self) -> Vec<u8> {
        let Address(value) = self;
        match hex::decode(value.trim_start_matches("0x")) {
            Ok(value) => value,
            Err(error) => panic!("A error has occured: {}", error),
        }
    }
    /// This method returns it as a string, for use when interacting with dispatchers
    pub fn to_string(self) -> String {
        let Address(value) = self;
        value
    }
}

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) async fn write_to_socket(path: String, content: &[u8]) -> io::Result<String> {
    let mut stream = UnixStream::connect(path).await?;

    stream.write_all(content).await?;
    let mut response = [0; 4096];
    let num_read = stream.read(&mut response).await?;
    let response = &response[..num_read];
    Ok(match String::from_utf8(response.to_vec()) {
        Ok(str) => str,
        Err(error) => panic!("an error has occured: {error:#?}"),
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
