#![doc = include_str!("../README.md")]

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

/// This module provides shared things throughout the crate
pub mod shared;

/// This module provides functions for getting information on the compositor
pub mod data;

/// This module provides the EventListener struct for listening and acting upon for events
pub mod event_listener;

/// This module is for calling dispatchers and changing keywords
pub mod dispatch;
