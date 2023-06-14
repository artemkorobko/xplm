use std::{ffi, path, str, string};

use thiserror::Error;

use super::ffi::FromStringBuf;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
pub enum UtilitiesError {
    /// Empty directory separator
    #[error("empty directory separator")]
    EmptyDirectorySeparator,
    /// Invalid output string passed from Rust to C
    // #[error("invalid output string {0}")]
    // InvalidOutputString(ffi::NulError),
    /// Invalid input string passed from C to Rust
    #[error("invalid input string {0}")]
    InvalidInputString(string::FromUtf8Error),
    /// Invalid input string slice passed from C to Rust
    #[error("invalid input string slice {0}")]
    InvalidInputStringSlice(str::Utf8Error),
}

/// Returns the full path to the X-System folder. Note that this is a directory path,
/// so it ends in a trailing : or / .
///
/// # Returns
/// Returns system path on success. Otherwise returns [`UtilitiesError::InvalidInputString`].
pub fn get_system_path() -> Result<path::PathBuf, UtilitiesError> {
    let mut buf = [0; 4096];
    unsafe { xplm_sys::XPLMGetSystemPath(buf.as_mut_ptr()) };
    String::from_string_buf(buf)
        .map(|path| path::PathBuf::from(&path))
        .map_err(UtilitiesError::InvalidInputString)
}

/// Returns a full path to a file that is within X-Plane’s preferences directory.
///
/// # Returns
/// Returns preferences file path on success. Otherwise returns [`UtilitiesError::InvalidInputString`].
pub fn get_prefs_path() -> Result<path::PathBuf, UtilitiesError> {
    let mut buf = [0; 4096];
    unsafe { xplm_sys::XPLMGetPrefsPath(buf.as_mut_ptr()) };
    String::from_string_buf(buf)
        .map(|path| path::PathBuf::from(&path))
        .map_err(UtilitiesError::InvalidInputString)
}

/// Returns a char that is the directory separator for the current platform.
/// The character returned will reflect the current file path mode.
///
/// # Returns
/// Returns directory separator on success.
/// Otherwise returns [`UtilitiesError::InvalidInputStringSlice`] or [`UtilitiesError::EmptyDirectorySeparator`].
pub fn get_directory_separator() -> Result<char, UtilitiesError> {
    unsafe { ffi::CStr::from_ptr(xplm_sys::XPLMGetDirectorySeparator()) }
        .to_str()
        .map_err(UtilitiesError::InvalidInputStringSlice)?
        .chars()
        .next()
        .ok_or(UtilitiesError::EmptyDirectorySeparator)
}
