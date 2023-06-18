use std::ffi;

/// An error returned from plugin API calls
#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    /// Invalid plugin ID
    #[error("invalid plugin id: {0}")]
    InvalidId(xplm_sys::XPLMPluginID),
    /// Invalid plugin path passed to X-Plane
    #[error("invalid plugin path string {0}")]
    InvalidPluginPath(ffi::NulError),
    /// Invalid plugin signature passed to X-Plane
    #[error("invalid plugin signature string {0}")]
    InvalidPluginSignature(ffi::NulError),
    /// Invalid plugin info name passed from X-Plane
    #[error("invalid plugin info name string {0}")]
    InvalidInfoName(ffi::IntoStringError),
    /// Invalid plugin info file path passed from X-Plane
    #[error("invalid plugin info file path string {0}")]
    InvalidInfoFilePath(ffi::IntoStringError),
    /// Invalid plugin info signature passed from X-Plane
    #[error("invalid plugin info signature string {0}")]
    InvalidInfoSignature(ffi::IntoStringError),
    /// Invalid plugin info description passed from X-Plane
    #[error("invalid plugin info description string {0}")]
    InvalidInfoDescription(ffi::IntoStringError),
}
