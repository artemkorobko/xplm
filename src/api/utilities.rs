use std::{ffi, path, str, string};

use thiserror::Error;

use super::ffi::FromStringBuf;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
pub enum UtilitiesError {
    /// Invalid system path string passed from X-Plane
    #[error("invalid system path {0}")]
    InvalidSystemPath(string::FromUtf8Error),
    /// Invalid preferences path string passed from X-Plane
    #[error("invalid preferences path {0}")]
    InvalidPrefsPath(string::FromUtf8Error),
    /// Invalid directory separator passed from X-Plane
    #[error("invalid directory separator {0}")]
    InvalidDirectorySeparator(str::Utf8Error),
    /// Sirectory separator is empty
    #[error("empty directory separator")]
    EmptyDirectorySeparator,
}

pub type Result<T> = std::result::Result<T, UtilitiesError>;

/// Returns the full path to the X-System folder. Note that this is a directory path,
/// so it ends in a trailing `:` or `/`.
///
/// # Returns
/// Returns system path on success. Otherwise returns [`UtilitiesError::InvalidSystemPath`].
pub fn get_system_path() -> Result<path::PathBuf> {
    let mut buf = [0; 4096];
    unsafe { xplm_sys::XPLMGetSystemPath(buf.as_mut_ptr()) };
    String::from_string_buf(buf)
        .map(|path| path::PathBuf::from(&path))
        .map_err(UtilitiesError::InvalidSystemPath)
}

/// Returns a full path to a file that is within X-Plane’s preferences directory.
///
/// # Returns
/// Returns preferences file path on success. Otherwise returns [`UtilitiesError::InvalidPrefsPath`].
pub fn get_prefs_path() -> Result<path::PathBuf> {
    let mut buf = [0; 4096];
    unsafe { xplm_sys::XPLMGetPrefsPath(buf.as_mut_ptr()) };
    String::from_string_buf(buf)
        .map(|path| path::PathBuf::from(&path))
        .map_err(UtilitiesError::InvalidPrefsPath)
}

/// Returns a char that is the directory separator for the current platform.
/// The character returned will reflect the current file path mode.
///
/// # Returns
/// Returns directory separator on success.
/// Otherwise returns [`UtilitiesError::InvalidDirectorySeparator`] or [`UtilitiesError::EmptyDirectorySeparator`].
pub fn get_directory_separator() -> Result<char> {
    unsafe { ffi::CStr::from_ptr(xplm_sys::XPLMGetDirectorySeparator()) }
        .to_str()
        .map_err(UtilitiesError::InvalidDirectorySeparator)?
        .chars()
        .next()
        .ok_or(UtilitiesError::EmptyDirectorySeparator)
}
