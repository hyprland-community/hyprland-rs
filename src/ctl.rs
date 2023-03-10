use derive_more::{Constructor, Display as MDisplay};
use std::fmt::Display as FDisplay;
use strum::{Display as SDisplay, EnumProperty};

use crate::shared::*;

/// Reload hyprland config
pub mod reload {
    use super::*;
    /// Reload hyprland config
    pub fn call() -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &"reload".to_string().into_bytes(),
        )?;
        Ok(())
    }
    /// Reload hyprland config (async)
    pub async fn call_async() -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &"reload".to_string().into_bytes(),
        )
        .await?;
        Ok(())
    }
}
/// Enter kill mode (similar to xkill)
pub mod kill {
    use super::*;
    /// Enter kill mode (similar to xkill)
    pub fn call() -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &"kill".to_string().into_bytes(),
        )?;
        Ok(())
    }
    /// Enter kill mode (similar to xkill) (async)
    pub async fn call_async() -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &"kill".to_string().into_bytes(),
        )
        .await?;
        Ok(())
    }
}

/// Set the cursor theme
pub mod set_cursor {
    use super::*;
    /// Set the cursor theme
    pub fn call<Str: FDisplay>(theme: Str, size: u16) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("setcursor {theme} {size}").into_bytes(),
        )?;
        Ok(())
    }
    /// Set the cursor theme (async)
    pub async fn call_async<Str: FDisplay>(theme: Str, size: u16) -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &format!("setcursor {theme} {size}").into_bytes(),
        )
        .await?;
        Ok(())
    }
}

/// Stuff related to managing virtual outputs/displays
pub mod output {
    use super::*;
    /// Output backend types
    #[derive(SDisplay)]
    pub enum OutputBackends {
        /// The wayland output backend
        #[strum(serialize = "wayland")]
        Wayland,
        /// The x11 output backend
        #[strum(serialize = "x11")]
        X11,
        /// The headless output backend
        #[strum(serialize = "headless")]
        Headless,
        /// Let Hyprland decide the backend type
        #[strum(serialize = "auto")]
        Auto,
    }

    /// Create virtual displays
    pub fn create(backend: OutputBackends) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("output create {backend}").into_bytes(),
        )?;
        Ok(())
    }
    /// Remove virtual displays
    pub fn remove<Str: FDisplay>(name: Str) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("output remove {name}").into_bytes(),
        )?;
        Ok(())
    }
}

/// Switch the xkb layout index for a keyboard
pub mod switch_xkb_layout {
    use super::*;
    /// The types of Cmds used by [switch_xkb_layout]
    #[derive(MDisplay)]
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
    pub fn call<Str: FDisplay>(device: Str, cmd: SwitchXKBLayoutCmdTypes) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("switchxkblayout {device} {cmd}").into_bytes(),
        )?;
        Ok(())
    }
    /// Switch the xkb layout index for a keyboard
    pub async fn call_async<Str: FDisplay>(
        device: Str,
        cmd: SwitchXKBLayoutCmdTypes,
    ) -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &format!("switchxkblayout {device} {cmd}").into_bytes(),
        )
        .await?;
        Ok(())
    }
}

/// Creates a error that Hyprland will display
pub mod set_error {
    use super::*;
    /// Creates a error that Hyprland will display
    pub fn call(color: Color, msg: String) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("seterror {color} {msg}").into_bytes(),
        )?;
        Ok(())
    }
    /// Creates a error that Hyprland will display (async)
    pub async fn call_async(color: Color, msg: String) -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &format!("seterror {color} {msg}").into_bytes(),
        )
        .await?;
        Ok(())
    }
}

/// A 8-bit color with a alpha channel
#[derive(MDisplay, Constructor)]
#[display(fmt = "rgba({},{},{},{})", "_0", "_1", "_2", "_3")]
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
    #[derive(EnumProperty, MDisplay)]
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
    pub fn call(ident: String, prop: PropType, lock: bool) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("setprop {ident} {prop} {}", if lock { "lock" } else { "" }).into_bytes(),
        )?;
        Ok(())
    }
    /// Sets a window prob (async)
    pub async fn call_async(ident: String, prop: PropType, lock: bool) -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &format!("setprop {ident} {prop} {}", if lock { "lock" } else { "" }).into_bytes(),
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
    pub fn load(path: &Path) -> HResult<()> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &format!("plugin load {}", path.display()).into_bytes(),
        )?;
        Ok(())
    }
    /// Loads a plugin, by path (async)
    pub async fn load_async(path: &Path) -> HResult<()> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &format!("plugin load {}", path.display()).into_bytes(),
        )
        .await?;
        Ok(())
    }
    /// Returns a list of all plugins
    pub fn list() -> HResult<String> {
        write_to_socket_sync(
            get_socket_path(SocketType::Command),
            &"plugin list".to_string().into_bytes(),
        )
    }
    /// Returns a list of all plugins (async)
    pub async fn list_async() -> HResult<String> {
        write_to_socket(
            get_socket_path(SocketType::Command),
            &"plugin list".to_string().into_bytes(),
        )
        .await
    }
}
