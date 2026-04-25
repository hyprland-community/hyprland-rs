mod error;
mod keyword;
mod monitor;
mod preload;
mod reload;
mod unload;
mod wallpaper;
mod wallpaper_listing;
mod wallpaper_mode;

use crate::default_instance;
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
pub fn hyprpaper(keyword: Keyword) -> crate::Result<Response> {
    instance_hyprpaper(default_instance()?, keyword)
}

/// Send a keyword to hyprpaper using IPC.
pub fn instance_hyprpaper(instance: &Instance, keyword: Keyword) -> crate::Result<Response> {
    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };
    let response = instance.write_to_hyprpaper_socket(content)?;
    process_hyprpaper_response(keyword, response)
}

/// Process a hyprpaper response (testable without socket).
fn process_hyprpaper_response(keyword: Keyword, response: String) -> crate::Result<Response> {
    keyword.expected_response().is_expected(response)
}

/// Send a keyword to hyprpaper using IPC.
pub async fn hyprpaper_async(keyword: Keyword) -> crate::Result<Response> {
    instance_hyprpaper_async(default_instance()?, keyword).await
}

/// Send a keyword to hyprpaper using IPC.
pub async fn instance_hyprpaper_async(
    instance: &Instance,
    keyword: Keyword,
) -> crate::Result<Response> {
    let content = CommandContent {
        flag: crate::shared::CommandFlag::Empty,
        data: keyword.to_string(),
    };
    let response = instance.write_to_hyprpaper_socket_async(content).await?;
    process_hyprpaper_response(keyword, response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_hyprpaper_response_ok() {
        let keyword = Keyword::Preload(Preload {
            path: "/foo".into(),
        });
        let result = process_hyprpaper_response(keyword, "ok".into());
        assert!(matches!(result, Ok(Response::Ok)));
    }

    #[test]
    fn test_process_hyprpaper_response_active() {
        let keyword = Keyword::ListActive;
        let result = process_hyprpaper_response(keyword, "DP-1 = /wallpaper.png".into());
        assert!(matches!(result, Ok(Response::ActiveWallpapers(_))));
    }

    #[test]
    fn test_process_hyprpaper_response_loaded() {
        let keyword = Keyword::ListLoaded;
        let result = process_hyprpaper_response(keyword, "/wallpaper.png".into());
        assert!(matches!(result, Ok(Response::LoadedWallpapers(_))));
    }

    #[test]
    fn test_process_hyprpaper_response_not_ok() {
        let keyword = Keyword::Preload(Preload {
            path: "/foo".into(),
        });
        let result = process_hyprpaper_response(keyword, "error".into());
        assert!(result.is_err());
    }
}
