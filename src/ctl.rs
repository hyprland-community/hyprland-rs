use derive_more::{Constructor, Display as MDisplay};
use std::fmt::Display as FDisplay;

use crate::shared::*;

/// Reload hyprland config
pub mod reload {
    use super::*;
    /// Reload hyprland config
    pub fn call() -> crate::Result<()> {
        write_to_socket_sync(SocketType::Command, command!(Empty, "reload"))?;
        Ok(())
    }
    /// Reload hyprland config (async)
    pub async fn call_async() -> crate::Result<()> {
        write_to_socket(SocketType::Command, command!(Empty, "reload")).await?;
        Ok(())
    }
}
/// Enter kill mode (similar to xkill)
pub mod kill {
    use super::*;
    /// Enter kill mode (similar to xkill)
    pub fn call() -> crate::Result<()> {
        write_to_socket_sync(SocketType::Command, command!(Empty, "kill"))?;
        Ok(())
    }
    /// Enter kill mode (similar to xkill) (async)
    pub async fn call_async() -> crate::Result<()> {
        write_to_socket(SocketType::Command, command!(Empty, "kill")).await?;
        Ok(())
    }
}

/// Set the cursor theme
pub mod set_cursor {
    use super::*;
    /// Set the cursor theme
    pub fn call<Str: FDisplay>(theme: Str, size: u16) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(Empty, "setcursor {theme} {size}"),
        )?;
        Ok(())
    }
    /// Set the cursor theme (async)
    pub async fn call_async<Str: FDisplay>(theme: Str, size: u16) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(Empty, "setcursor {theme} {size}"),
        )
        .await?;
        Ok(())
    }
}

/// Stuff related to managing virtual outputs/displays
pub mod output {
    use super::*;
    /// Output backend types
    #[derive(Debug, MDisplay, Clone, Copy, PartialEq, Eq)]
    pub enum OutputBackends {
        /// The wayland output backend
        #[display("wayland")]
        Wayland,
        /// The x11 output backend
        #[display("x11")]
        X11,
        /// The headless output backend
        #[display("headless")]
        Headless,
        /// Let Hyprland decide the backend type
        #[display("auto")]
        Auto,
    }

    /// Create virtual displays
    pub fn create(backend: OutputBackends) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(Empty, "output create {backend}"),
        )?;
        Ok(())
    }
    /// Remove virtual displays
    pub fn remove<Str: FDisplay>(name: Str) -> crate::Result<()> {
        write_to_socket_sync(SocketType::Command, command!(Empty, "output remove {name}"))?;
        Ok(())
    }
}

/// Switch the xkb layout index for a keyboard
pub mod switch_xkb_layout {
    use super::*;
    /// The types of Cmds used by [switch_xkb_layout]
    #[derive(Debug, MDisplay, Clone, Copy, PartialEq, Eq)]
    pub enum SwitchXKBLayoutCmdTypes {
        /// Next input
        #[display("next")]
        Next,
        /// Previous inout
        #[display("prev")]
        Previous,
        /// Set to a specific input id
        #[display("{_0}")]
        Id(u8),
    }

    /// Switch the xkb layout index for a keyboard
    pub fn call<Str: FDisplay>(device: Str, cmd: SwitchXKBLayoutCmdTypes) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(Empty, "switchxkblayout {device} {cmd}"),
        )?;
        Ok(())
    }
    /// Switch the xkb layout index for a keyboard
    pub async fn call_async<Str: FDisplay>(
        device: Str,
        cmd: SwitchXKBLayoutCmdTypes,
    ) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(Empty, "switchxkblayout {device} {cmd}"),
        )
        .await?;
        Ok(())
    }
}

/// Creates a error that Hyprland will display
pub mod set_error {
    use super::*;
    /// Creates a error that Hyprland will display
    pub fn call(color: Color, msg: String) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(Empty, "seterror {color} {msg}"),
        )?;
        Ok(())
    }
    /// Creates a error that Hyprland will display (async)
    pub async fn call_async(color: Color, msg: String) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(Empty, "seterror {color} {msg}"),
        )
        .await?;
        Ok(())
    }
}

/// Creates a notification with Hyprland
pub mod notify {
    use super::*;
    use std::time::Duration;

    #[allow(missing_docs)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(i8)]
    pub enum Icon {
        NoIcon = -1,
        Warning = 0,
        Info = 1,
        Hint = 2,
        Error = 3,
        Confused = 4,
        Ok = 5,
    }
    /// Creates a notification with Hyprland
    pub fn call(icon: Icon, time: Duration, color: Color, msg: String) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(
                Empty,
                "notify {} {} {color} {msg}",
                icon as i8,
                time.as_millis()
            ),
        )?;
        Ok(())
    }
    /// Creates a error that Hyprland will display (async)
    pub async fn call_async(
        icon: Icon,
        time: Duration,
        color: Color,
        msg: String,
    ) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(
                Empty,
                "notify {} {} {color} {msg}",
                icon as i8,
                time.as_millis()
            ),
        )
        .await?;
        Ok(())
    }
}
/// Dismisses all or up to a specified amount of notifications with Hyprland
pub mod dismissnotify {
    use std::num::NonZeroU8;

    use super::*;
    /// Dismisses notifications with Hyprland
    ///
    /// If `amount` is [None] then will dismiss ALL notifications
    pub fn call(amount: Option<NonZeroU8>) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(
                Empty,
                "dismissnotify {}",
                if let Some(amount) = amount {
                    amount.to_string()
                } else {
                    (-1).to_string()
                }
            ),
        )?;
        Ok(())
    }
    /// Dismisses notifications with Hyprland (async)
    ///
    /// If `amount` is [None] then will dismiss ALL notifications
    pub async fn call_async(amount: Option<NonZeroU8>) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(
                Empty,
                "dismissnotify {}",
                if let Some(amount) = amount {
                    amount.to_string()
                } else {
                    (-1).to_string()
                }
            ),
        )
        .await?;
        Ok(())
    }
}

/// A 8-bit color with a alpha channel
#[derive(Debug, Copy, Clone, MDisplay, Constructor, PartialEq, Eq)]
#[display("rgba({_0:02x}{_1:02x}{_2:02x}{_3:02x})")]
pub struct Color(u8, u8, u8, u8);

/// Provides things to setting props
pub mod set_prop {
    use super::*;

    fn l(b: bool) -> &'static str {
        if b {
            "lock"
        } else {
            ""
        }
    }

    /// Type that represents a prop
    #[derive(MDisplay, Clone, PartialEq)]
    pub enum PropType {
        /// The animation style
        #[display("animationstyle {_0}")]
        AnimationStyle(String),
        /// The roundness
        #[display("rounding {_0} {}", l(*_1))]
        Rounding(
            i64,
            /// locked
            bool,
        ),
        /// Force no blur
        #[display("forcenoblur {} {}", *_0 as u8, l(*_1))]
        ForceNoBlur(
            bool,
            /// locked
            bool,
        ),
        /// Force opaque
        #[display("forceopaque {} {}", *_0 as u8, l(*_1))]
        ForceOpaque(
            bool,
            /// locked
            bool,
        ),
        /// Force opaque overriden
        #[display("forceopaqueoverriden {} {}", *_0 as u8, l(*_1))]
        ForceOpaqueOverriden(
            bool,
            /// locked
            bool,
        ),
        /// Force allow input
        #[display("forceallowsinput {} {}", *_0 as u8, l(*_1))]
        ForceAllowsInput(
            bool,
            /// locked
            bool,
        ),
        /// Force no animations
        #[display("forcenoanims {} {}", *_0 as u8, l(*_1))]
        ForceNoAnims(
            bool,
            /// locked
            bool,
        ),
        /// Force no border
        #[display("forcenoborder {} {}", *_0 as u8, l(*_1))]
        ForceNoBorder(
            bool,
            /// locked
            bool,
        ),
        /// Force no shadow
        #[display("forcenoshadow {} {}", *_0 as u8, l(*_1))]
        ForceNoShadow(
            bool,
            /// locked
            bool,
        ),
        /// Allow for windoe dancing?
        #[display("windowdancecompat {} {}", *_0 as u8, l(*_1))]
        WindowDanceCompat(
            bool,
            /// locked
            bool,
        ),
        /// Allow for overstepping max size
        #[display("nomaxsize {} {}", *_0 as u8, l(*_1))]
        NoMaxSize(
            bool,
            /// locked
            bool,
        ),
        /// Dim around?
        #[display("dimaround {} {}", *_0 as u8, l(*_1))]
        DimAround(
            bool,
            /// locked
            bool,
        ),
        /// Makes the next setting be override instead of multiply
        #[display("alphaoverride {} {}", *_0 as u8, l(*_1))]
        AlphaOverride(
            bool,
            /// locked
            bool,
        ),
        /// The alpha
        #[display("alpha {_0} {}", l(*_1))]
        Alpha(
            f32,
            /// locked
            bool,
        ),
        /// Makes the next setting be override instead of multiply
        #[display("alphainactiveoverride {} {}", *_0 as u8, l(*_1))]
        AlphaInactiveOverride(
            bool,
            /// locked
            bool,
        ),
        /// The alpha for inactive
        #[display("alphainactive {_0} {}", l(*_1))]
        AlphaInactive(
            f32,
            /// locked
            bool,
        ),
        /// The active border color
        #[display("alphabordercolor {_0} {}", l(*_1))]
        ActiveBorderColor(
            Color,
            /// locked
            bool,
        ),
        /// The inactive border color
        #[display("inalphabordercolor {_0} {}", l(*_1))]
        InactiveBorderColor(
            Color,
            /// locked
            bool,
        ),
    }

    /// Sets a window prob
    pub fn call(ident: String, prop: PropType, lock: bool) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(
                Empty,
                "setprop {ident} {prop} {}",
                if lock { "lock" } else { "" }
            ),
        )?;
        Ok(())
    }
    /// Sets a window prob (async)
    pub async fn call_async(ident: String, prop: PropType, lock: bool) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(
                Empty,
                "setprop {ident} {prop} {}",
                if lock { "lock" } else { "" }
            ),
        )
        .await?;
        Ok(())
    }
}

/// Provides functions for communication with plugin system
pub mod plugin {
    use super::*;
    use std::path::Path;

    /// Loads a plugin, by path
    pub fn load(path: &Path) -> crate::Result<()> {
        write_to_socket_sync(
            SocketType::Command,
            command!(Empty, "plugin load {}", path.display()),
        )?;
        Ok(())
    }
    /// Loads a plugin, by path (async)
    pub async fn load_async(path: &Path) -> crate::Result<()> {
        write_to_socket(
            SocketType::Command,
            command!(Empty, "plugin load {}", path.display()),
        )
        .await?;
        Ok(())
    }
    /// Returns a list of all plugins
    pub fn list() -> crate::Result<String> {
        write_to_socket_sync(SocketType::Command, command!(Empty, "plugin list"))
    }
    /// Returns a list of all plugins (async)
    pub async fn list_async() -> crate::Result<String> {
        write_to_socket(SocketType::Command, command!(Empty, "plugin list")).await
    }
}
