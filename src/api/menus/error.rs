use std::ffi;

/// An error returned from menu API calls.
#[derive(thiserror::Error, Debug)]
pub enum MenusError {
    /// Invalid menu ID.
    #[error("invalid menu id")]
    InvalidId,
    /// Invalid menu item ID.
    #[error("invalid menu item id")]
    InvalidMenuItemId,
    /// Invalid menu name string passed to X-Plane.
    #[error("invalid menu name {0}")]
    InvalidMenuName(ffi::NulError),
    /// Unknown menu item state.
    #[error("unknown menu item state {0}")]
    UnknownMenuItemState(xplm_sys::XPLMMenuCheck),
}
