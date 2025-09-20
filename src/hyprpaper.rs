mod error;
mod keyword;
mod monitor;
mod preload;
mod reload;
mod unload;
mod wallpaper;
mod wallpaper_listing;
mod wallpaper_mode;

pub use error::Error;
pub use keyword::Keyword;
pub use monitor::Monitor;
pub use preload::Preload;
pub use reload::Reload;
pub use unload::Unload;
pub use wallpaper::Wallpaper;
pub use wallpaper_listing::WallpaperListing;
pub use wallpaper_mode::WallpaperMode;

use crate::shared::{write_to_socket, write_to_socket_sync, CommandContent, SocketType};

/// Response from hyprpaper.
pub enum Response {
    /// Keyword was accepted.
    Ok,
    /// A list of active wallpapers.
    ActiveWallpapers(Vec<WallpaperListing>),
    /// A list of loaded wallpapers.
    LoadedWallpapers(Vec<String>),
}

/// Send a keyword to hyprpaper using IPC.
pub fn hyprpaper(keyword: Keyword) -> crate::Result<Response> {
    let expected_response = keyword.expected_response();

    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };

    let response = write_to_socket_sync(SocketType::Hyprpaper, content)?;

    expected_response.is_expected(response)
}

/// Send a keyword to hyprpaper using IPC.
pub async fn hyprpaper_async(keyword: Keyword) -> crate::Result<Response> {
    let expected_response = keyword.expected_response();

    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };

    let response = write_to_socket(SocketType::Hyprpaper, content).await?;

    expected_response.is_expected(response)
}
