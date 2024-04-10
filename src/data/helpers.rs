use super::*;

/// A helper struct that provides the current fullscreen state
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct FullscreenState(
    /// State
    pub bool,
);

impl HyprData for FullscreenState {
    fn get() -> crate::Result<Self> {
        Ok(Self(Workspace::get_active()?.fullscreen))
    }
    async fn get_async() -> crate::Result<Self> {
        Ok(Self(Workspace::get_active_async().await?.fullscreen))
    }
}

impl FullscreenState {
    /// This method returns a bool of the current fullscreen state
    pub fn bool(self) -> bool {
        self.0
    }
}
