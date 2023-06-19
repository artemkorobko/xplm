use std::{ffi, str};

/// An error returned from utilities API calls
#[derive(thiserror::Error, Debug)]
pub enum UtilitiesError {
    /// Invalid system path string returned from X-Plane
    #[error("invalid system path {0}")]
    InvalidSystemPath(ffi::IntoStringError),
    /// Invalid preferences path string returned from X-Plane
    #[error("invalid preferences path {0}")]
    InvalidPrefsPath(ffi::IntoStringError),
    /// Invalid directory separator returned from X-Plane
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
    LoadDataFile,
    /// Unable to load data file
    #[error("unable to clear replay")]
    ClearReplay,
    /// Unable to save data file
    #[error("unable to save data file")]
    SaveDataFile,
    /// Unknown host application id
    #[error("unknown host application id {0}")]
    UnknownHostApplicationId(xplm_sys::XPLMHostApplicationID),
    /// Unknown language code
    #[error("unknown language code {0}")]
    UnknownLanguageCode(xplm_sys::XPLMLanguageCode),
    /// Invalid virtual key returned from X-Plane
    #[error("invalid virtual key {0}")]
    InvalidVirtualKey(::std::os::raw::c_char),
    /// Invalid virtual key description string returned from X-Plane
    #[error("invalid virtual key description {0}")]
    InvalidVirtualKeyDescription(ffi::IntoStringError),
    /// Invalid command reference
    #[error("invalid command reference")]
    InvalidCommand,
    /// Invalid command name string passed to X-Plane
    #[error("invalid command name {0}")]
    InvalidCommandName(ffi::NulError),
    /// Invalid command description string passed to X-Plane
    #[error("invalid command description {0}")]
    InvalidCommandDescription(ffi::NulError),
}
