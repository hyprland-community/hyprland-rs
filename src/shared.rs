//! # The Shared Module
//!
//! This module provides shared private and public functions, structs, enum, and types
use derive_more::Display;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env::{var, VarError};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::{error, fmt, io};

#[derive(Debug, derive_more::Display)]
/// Error that unifies different error types used by Hyprland-rs
pub enum HyprError {
    /// Error coming from serde
    #[display(format = "{_0}")]
    SerdeError(serde_json::Error),
    /// Error coming from std::io
    #[display(format = "{_0}")]
    IoError(io::Error),
    /// Error that occurs when parsing UTF-8 string
    #[display(format = "{_0}")]
    FromUtf8Error(std::string::FromUtf8Error),
    /// Dispatcher returned non `ok` value
    #[display(format = "A dispatcher returned a non-`ok`, value which is probably an error: {_0}")]
    NotOkDispatch(String),
    /// Internal Hyprland error
    Internal(String),
    /// Error that occurs for other reasons. Avoid using this.
    #[display(format = "{_0}")]
    Other(String),
}
impl HyprError {
    /// Try to get an owned version of the internal error.
    ///
    /// Some dependencies of hyprland do not impl Clone in their error types. This is a partial workaround.
    ///
    /// If it succeeds, it returns the owned version of HyprError in Ok(). Otherwise, it returns a reference to the error type.
    pub fn try_as_cloned(&self) -> Result<Self, &Self> {
        match self {
            Self::SerdeError(_) => Err(self),
            Self::IoError(_) => Err(self),
            Self::FromUtf8Error(e) => Ok(Self::FromUtf8Error(e.clone())),
            Self::NotOkDispatch(s) => Ok(Self::NotOkDispatch(s.clone())),
            Self::Internal(s) => Ok(Self::Internal(s.clone())),
            Self::Other(s) => Ok(Self::Other(s.clone())),
        }
    }
    /// Create a Hyprland error with dynamic data.
    #[inline(always)]
    pub fn other<S: Into<String>>(other: S) -> Self {
        Self::Other(other.into())
    }
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

impl error::Error for HyprError {}

/// Internal macro to return a Hyprland error
macro_rules! hypr_err {
    ($fmt:literal) => {
        return Err($crate::shared::HyprError::Internal(format!($fmt)))
    };
    (other $fmt:literal) => {
        return Err($crate::shared::HyprError::Other(format!($fmt)))
    };
    ($fmt:literal $(, $value:expr)+) => {
        return Err($crate::shared::HyprError::Internal(format!($fmt $(, $value)+)))
    };
    (other $fmt:literal $(, $value:expr)+) => {
        return Err($crate::shared::HyprError::Other(format!($fmt $(, $value)+)))
    };
}

pub(crate) use hypr_err;

/// This type provides the result type used everywhere in Hyprland-rs
#[deprecated(since = "0.3.1", note = "New location: hyprland::Result")]
pub type HResult<T> = Result<T, HyprError>;

/// The address struct holds a address as a tuple with a single value
/// and has methods to reveal the address in different data formats
#[derive(
    Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, derive_more::Display,
)]
pub struct Address(String);
impl Address {
    #[inline(always)]
    pub(crate) fn fmt_new(address: &str) -> Self {
        // this way is faster than std::fmt
        Self("0x".to_owned() + address)
    }
    /// This creates a new address from a value that implements [std::string::ToString]
    pub fn new<T: ToString>(string: T) -> Self {
        Self(string.to_string())
    }
}

/// This trait provides a standardized way to get data
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

#[inline]
fn ser_spec_opt(opt: &Option<String>) -> String {
    match opt {
        Some(name) => "special:".to_owned() + name,
        None => "special".to_owned(),
    }
}

/// This enum holds workspace data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(untagged)]
pub enum WorkspaceType {
    /// A named workspace
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
macro_rules! from {
    ($($ty:ty),+$(,)?) => {
        $(
            impl TryFrom<$ty> for WorkspaceType {
                type Error = HyprError;
                fn try_from(int: $ty) -> Result<Self, Self::Error> {
                    match int {
                        1.. => Ok(WorkspaceType::Regular(int.to_string())),
                        _ => hypr_err!("Conversion error: Unrecognised id"),
                    }
                }
            }
        )+
    };
}
from![u8, u16, u32, u64, usize, i8, i16, i32, i64, isize];

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

/// This pub(crate) function is used to write a value to a socket and to get the response
pub(crate) async fn write_to_socket(
    ty: SocketType,
    content: CommandContent,
) -> crate::Result<String> {
    use crate::unix_async::*;

    let path = get_socket_path(ty)?;
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
pub(crate) fn write_to_socket_sync(
    ty: SocketType,
    content: CommandContent,
) -> crate::Result<String> {
    use io::prelude::*;
    use std::os::unix::net::UnixStream;

    let path = get_socket_path(ty)?;
    let mut stream = UnixStream::connect(path)?;

    stream.write_all(&content.as_bytes())?;

    let mut response = Vec::new();

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SocketType {
    /// The socket used to send commands to Hyprland (AKA `.socket.sock`)
    Command,
    /// The socket used to listen for events (AKA `.socket2.sock`)
    Listener,
}
impl SocketType {
    pub(crate) const fn socket_name(&self) -> &'static str {
        match self {
            Self::Command => ".socket.sock",
            Self::Listener => ".socket2.sock",
        }
    }
}

pub(crate) static COMMAND_SOCK: Lazy<crate::Result<PathBuf>> =
    Lazy::new(|| init_socket_path(SocketType::Command));
pub(crate) static LISTENER_SOCK: Lazy<crate::Result<PathBuf>> =
    Lazy::new(|| init_socket_path(SocketType::Listener));

/// Get the socket path. According to benchmarks, this is faster than an atomic OnceCell.
pub(crate) fn get_socket_path(socket_type: SocketType) -> crate::Result<PathBuf> {
    macro_rules! me {
        ($var:expr) => {
            match $var {
                Ok(p) => Ok(p.clone()),
                Err(e) => Err(match e.try_as_cloned() {
                    Ok(c) => c,
                    Err(e) => HyprError::Other(e.to_string()),
                }),
            }
        };
    }
    match socket_type {
        SocketType::Command => me!(COMMAND_SOCK.as_ref()),
        SocketType::Listener => me!(LISTENER_SOCK.as_ref()),
    }
}

fn init_socket_path(socket_type: SocketType) -> crate::Result<PathBuf> {
    let instance = match var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(var) => var,
        Err(VarError::NotPresent) => {
            hypr_err!("Could not get socket path! (Is Hyprland running??)")
        }
        Err(VarError::NotUnicode(_)) => {
            hypr_err!("Corrupted Hyprland socket variable: Invalid unicode!")
        }
    };

    let mut p: PathBuf;
    fn var_path(instance: String) -> Option<PathBuf> {
        if let Ok(runtime_path) = var("XDG_RUNTIME_DIR") {
            let mut buf = PathBuf::from(runtime_path);
            buf.push("hypr");
            buf.push(instance);
            if buf.exists() {
                return Some(buf);
            }
        }
        None
    }
    fn uid_path(instance: String) -> Option<PathBuf> {
        if let Ok(uid) = var("UID") {
            let mut buf = PathBuf::from("/run/user/".to_owned() + &uid);
            buf.push("hypr");
            buf.push(instance);
            if buf.exists() {
                return Some(buf);
            }
        }
        None
    }
    let old_buf = PathBuf::from("/tmp/hypr/".to_owned() + &instance);
    if let Some(path) = var_path(instance.clone()) {
        p = path;
    } else if let Some(path) = uid_path(instance) {
        p = path;
    } else if old_buf.exists() {
        p = old_buf;
    } else {
        hypr_err!("No xdg runtime path found!")
    }

    p.push(socket_type.socket_name());
    Ok(p)
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandFlag {
    /// The JSON flag.
    #[default]
    JSON,
    /// An empty flag.
    Empty,
}

/// This struct defines the content of a command, which consists of a flag and a data string.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
