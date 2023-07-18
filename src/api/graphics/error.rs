use std::ffi;

/// An error returned from graphics API calls.
#[derive(thiserror::Error, Debug)]
pub enum GraphicsError {
    /// Invalid window title string passed to X-Plane.
    #[error("invalid string {0}")]
    InvalidString(ffi::NulError),
}
