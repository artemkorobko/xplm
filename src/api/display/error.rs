use std::ffi;

/// An error returned from display API calls
#[derive(thiserror::Error, Debug)]
pub enum DisplayError {
    /// Invalid window id returned from X-Plane
    #[error("invalid window id")]
    InvalidWindowId,
    /// Invalid command name string passed to X-Plane
    #[error("invalid command name {0}")]
    InvalidCommandName(ffi::NulError),
    /// Unknown mouse status passed from X-Plane
    #[error("unknown mouse status {0}")]
    UnknownMouseStatuts(xplm_sys::XPLMMouseStatus),
    /// Unknown mouse wheel axis passed from X-Plane
    #[error("unknown mouse wheel axis {0}")]
    UnknownMouseWheelAxis(::std::os::raw::c_int),
}
