use std::{
    ffi::{self, CString},
    path, str, string,
};

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
    /// Invalid data file path string passed to X-Plane
    #[error("invalid system path")]
    InvalidDataFilePath(ffi::NulError),
    /// Unable to load data file
    #[error("unable to load data file")]
    LoadDataFileError,
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

/// Types of data files you can load or unload using the SDK
#[repr(i32)]
pub enum DataFileType {
    /// A situation (.sit) file, which starts off a flight in a given configuration.
    Situation = 1,
    /// A situation movie (.smo) file, which replays a past flight.
    ReplayMovie = 2,
}

/// Loads a data file of a given type. Paths must be relative to the X-System folder.
pub fn load_data_file<P: AsRef<path::Path>>(file_type: DataFileType, file_path: P) -> Result<()> {
    let file_path_str = file_path
        .as_ref()
        .to_str()
        .ok_or(UtilitiesError::LoadDataFileError)?;
    let c_string = CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    if unsafe { xplm_sys::XPLMLoadDataFile(file_type as i32, c_string.as_ptr()) == 1 } {
        Ok(())
    } else {
        Err(UtilitiesError::LoadDataFileError)
    }
}
