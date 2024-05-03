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
        #[display(fmt = "wayland")]
        Wayland,
        /// The x11 output backend
        #[display(fmt = "x11")]
        X11,
        /// The headless output backend
        #[display(fmt = "headless")]
        Headless,
        /// Let Hyprland decide the backend type
        #[display(fmt = "auto")]
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
        #[display(fmt = "next")]
        Next,
        /// Previous inout
        #[display(fmt = "prev")]
        Previous,
        /// Set to a specific input id
        #[display(fmt = "{}", "_0")]
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

/// A 8-bit color with a alpha channel
#[derive(Debug, Copy, Clone, MDisplay, Constructor, PartialEq, Eq)]
#[display(fmt = "rgba({:02x}{:02x}{:02x}{:02x})", "_0", "_1", "_2", "_3")]
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
        #[display(fmt = "animationstyle {}", "_0")]
        AnimationStyle(String),
        /// The roundness
        #[display(fmt = "rounding {} {}", "_0", "l(*_1)")]
        Rounding(
            i64,
            /// locked
            bool,
        ),
        /// Force no blur
        #[display(fmt = "forcenoblur {} {}", "*_0 as u8", "l(*_1)")]
        ForceNoBlur(
            bool,
            /// locked
            bool,
        ),
        /// Force opaque
        #[display(fmt = "forceopaque {} {}", "*_0 as u8", "l(*_1)")]
        ForceOpaque(
            bool,
            /// locked
            bool,
        ),
        /// Force opaque overriden
        #[display(fmt = "forceopaqueoverriden {} {}", "*_0 as u8", "l(*_1)")]
        ForceOpaqueOverriden(
            bool,
            /// locked
            bool,
        ),
        /// Force allow input
        #[display(fmt = "forceallowsinput {} {}", "*_0 as u8", "l(*_1)")]
        ForceAllowsInput(
            bool,
            /// locked
            bool,
        ),
        /// Force no animations
        #[display(fmt = "forcenoanims {} {}", "*_0 as u8", "l(*_1)")]
        ForceNoAnims(
            bool,
            /// locked
            bool,
        ),
        /// Force no border
        #[display(fmt = "forcenoborder {} {}", "*_0 as u8", "l(*_1)")]
        ForceNoBorder(
            bool,
            /// locked
            bool,
        ),
        /// Force no shadow
        #[display(fmt = "forcenoshadow {} {}", "*_0 as u8", "l(*_1)")]
        ForceNoShadow(
            bool,
            /// locked
            bool,
        ),
        /// Allow for windoe dancing?
        #[display(fmt = "windowdancecompat {} {}", "*_0 as u8", "l(*_1)")]
        WindowDanceCompat(
            bool,
            /// locked
            bool,
        ),
        /// Allow for overstepping max size
        #[display(fmt = "nomaxsize {} {}", "*_0 as u8", "l(*_1)")]
        NoMaxSize(
            bool,
            /// locked
            bool,
        ),
        /// Dim around?
        #[display(fmt = "dimaround {} {}", "*_0 as u8", "l(*_1)")]
        DimAround(
            bool,
            /// locked
            bool,
        ),
        /// Makes the next setting be override instead of multiply
        #[display(fmt = "alphaoverride {} {}", "*_0 as u8", "l(*_1)")]
        AlphaOverride(
            bool,
            /// locked
            bool,
        ),
        /// The alpha
        #[display(fmt = "alpha {} {}", "_0", "l(*_1)")]
        Alpha(
            f32,
            /// locked
            bool,
        ),
        /// Makes the next setting be override instead of multiply
        #[display(fmt = "alphainactiveoverride {} {}", "*_0 as u8", "l(*_1)")]
        AlphaInactiveOverride(
            bool,
            /// locked
            bool,
        ),
        /// The alpha for inactive
        #[display(fmt = "alphainactive {} {}", "_0", "l(*_1)")]
        AlphaInactive(
            f32,
            /// locked
            bool,
        ),
        /// The active border color
        #[display(fmt = "alphabordercolor {} {}", "_0", "l(*_1)")]
        ActiveBorderColor(
            Color,
            /// locked
            bool,
        ),
        /// The inactive border color
        #[display(fmt = "inalphabordercolor {} {}", "_0", "l(*_1)")]
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
