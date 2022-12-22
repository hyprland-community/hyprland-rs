use super::*;

/// A helper struct that provides the current fullscreen state
pub struct FullscreenState(
    /// State
    pub bool,
);

#[async_trait]
impl HyprData for FullscreenState {
    fn get() -> HResult<Self> {
        Ok(Self(Workspace::get_active()?.fullscreen))
    }
    async fn get_async() -> HResult<Self> {
        Ok(Self(Workspace::get_active_async().await?.fullscreen))
    }
}

impl FullscreenState {
    /// This method returns a bool of the current fullscreen state
    pub fn bool(self) -> bool {
        self.0
    }
}
