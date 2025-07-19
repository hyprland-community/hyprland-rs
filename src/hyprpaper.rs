mod error;
mod keyword;
mod monitor;
mod preload;
mod reload;
mod unload;
mod wallpaper;
mod wallpaper_listing;
mod wallpaper_mode;

use crate::instance::Instance;
use crate::shared::CommandContent;
pub use error::Error;
pub use keyword::Keyword;
pub use monitor::Monitor;
pub use preload::Preload;
pub use reload::Reload;
pub use unload::Unload;
pub use wallpaper::Wallpaper;
pub use wallpaper_listing::WallpaperListing;
pub use wallpaper_mode::WallpaperMode;

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
pub fn hyprpaper(instance: &Instance, keyword: Keyword) -> crate::Result<Response> {
    let expected_response = keyword.expected_response();

    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };
    let response = instance.write_to_hyprpaper_socket(content)?;
    expected_response.is_expected(response)
}

/// Send a keyword to hyprpaper using IPC.
pub async fn hyprpaper_async(instance: &Instance, keyword: Keyword) -> crate::Result<Response> {
    let expected_response = keyword.expected_response();

    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };
    let response = instance.write_to_hyprpaper_socket_async(content).await?;
    expected_response.is_expected(response)
}
