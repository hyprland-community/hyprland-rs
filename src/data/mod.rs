//! # Data module
//!
//! This module provides functions for getting information on the compositor
//!
//! ## Usage
//!
//! here is a example of every function in use! (blocking)
//! ```rust
//! use hyprland::data::*;
//! use hyprland::prelude::*;
//! use hyprland::Result;
//!
//! fn main() -> Result<()> {
//!     let instance = &hyprland::instance::Instance::from_current_env()?;
//!
//!     let monitors = Monitors::get(instance)?.to_vec();
//!     println!("{monitors:#?}");
//!
//!     let workspaces = Workspaces::get(instance)?.to_vec();
//!     println!("{workspaces:#?}");
//!
//!     let clients = Clients::get(instance)?.to_vec();
//!     println!("{clients:#?}");
//!
//!     let active_window = Client::get_active(instance)?;
//!     println!("{active_window:#?}");
//!
//!     let layers = Layers::get(instance)?;
//!     println!("{layers:#?}");
//!
//!     let devices = Devices::get(instance)?;
//!     println!("{devices:#?}");
//!
//!     let version = Version::get(instance)?;
//!     println!("{version:#?}");
//!
//!     let cursor_pos = CursorPosition::get(instance)?;
//!     println!("{cursor_pos:#?}");
//!     Ok(())
//! }
//! ```

#[macro_use]
mod macros;

use crate::shared::*;

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

mod regular;

/// Helpers data commands, these use other hyprctl commands to create new ones!
mod helpers;

pub use crate::data::helpers::*;

pub use crate::data::regular::*;

//// This module provides async function calls
//pub mod asynchronous;

//// This module provides blocking function calls
//pub mod blocking;
