//! # Data module
//!
//! This module provides functions for getting information on the compositor
//!
//! ## Usage
//!
//! here is a example of every function in use! (blocking)
//! ```rust
//! use hyprland::data::blocking::{
//!     get_monitors,
//!     get_workspaces,
//!     get_clients,
//!     get_active_window,
//!     get_layers,
//!     get_devices
//! };
//!
//! fn main() -> std::io::Result<()> {
//!     let monitors = get_monitors()?;
//!     println!("{monitors:#?}");
//!
//!     let workspaces = get_workspaces()?;
//!     println!("{workspaces:#?}");
//!
//!     let clients = get_clients()?;
//!     println!("{clients:#?}");
//!
//!     let active_window = get_active_window()?;
//!     println!("{active_window:#?}");
//!
//!     let layers = get_layers()?;
//!     println!("{layers:#?}");
//!
//!     let devices = get_devices()?;
//!     println!("{devices:#?}");
//!
//!     Ok(())
//! }
//! ```

mod shared;

pub use crate::data::shared::*;

/// This module provides async function calls
pub mod asynchronous;

/// This module provides blocking function calls
pub mod blocking;
