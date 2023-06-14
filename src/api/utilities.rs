use std::{ffi, path, str, string, sync::OnceLock};

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
    /// Unable to save data file
    #[error("unable to save data file")]
    SaveDataFileError,
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

/// Types of data files you can load or unload using the SDK.
#[repr(i32)]
pub enum DataFileType {
    /// A situation (.sit) file, which starts off a flight in a given configuration.
    Situation = 1,
    /// A situation movie (.smo) file, which replays a past flight.
    ReplayMovie = 2,
}

/// Loads a data file of a given type.
///
/// # Arguments
/// * `file_type` - the type of the file to load. See [`DataFileType`].
/// * `file_path` - the file path that must be relative to the X-System folder.
///
/// # Returns
/// Returns `Ok` in case of success. Otherwise returns
/// * [`UtilitiesError::LoadDataFileError`] if data file can't be loaded.
/// * [`UtilitiesError::InvalidDataFilePath`] if file_path contains invalid characters.
pub fn load_data_file<P: AsRef<path::Path>>(file_type: DataFileType, file_path: P) -> Result<()> {
    let file_path_str = file_path
        .as_ref()
        .to_str()
        .ok_or(UtilitiesError::LoadDataFileError)?;
    let c_string = ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    if unsafe { xplm_sys::XPLMLoadDataFile(file_type as i32, c_string.as_ptr()) == 1 } {
        Ok(())
    } else {
        Err(UtilitiesError::LoadDataFileError)
    }
}

/// Clears the replay. This is only valid with replay movies, not sit files.
///
/// # Returns
/// Returns `Ok` in case of success. Otherwise returns [`UtilitiesError::LoadDataFileError`].
pub fn clear_replay() -> Result<()> {
    if unsafe {
        xplm_sys::XPLMLoadDataFile(DataFileType::ReplayMovie as i32, std::ptr::null_mut()) == 1
    } {
        Ok(())
    } else {
        Err(UtilitiesError::LoadDataFileError)
    }
}

/// Saves the current situation or replay.
///
/// # Arguments
/// * `file_type` - the type of the file to save. See [`DataFileType`].
/// * `file_path` - the file path that must be relative to the X-System folder.
///
/// # Returns
/// Returns `Ok` in case of success. Otherwise returns
/// * [`UtilitiesError::SaveDataFileError`] if data file can't be loaded.
/// * [`UtilitiesError::InvalidDataFilePath`] if file_path contains invalid characters.
pub fn save_data_file<P: AsRef<path::Path>>(file_type: DataFileType, file_path: P) -> Result<()> {
    let file_path_str = file_path
        .as_ref()
        .to_str()
        .ok_or(UtilitiesError::SaveDataFileError)?;
    let c_string = ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    if unsafe { xplm_sys::XPLMSaveDataFile(file_type as i32, c_string.as_ptr()) == 1 } {
        Ok(())
    } else {
        Err(UtilitiesError::SaveDataFileError)
    }
}

/// While the plug-in SDK is only accessible to plugins running inside X-Plane,
/// the original authors considered extending the API to other applications that
/// shared basic infrastructure with X-Plane. These enumerations are hold-overs
/// from that original roadmap; all values other than X-Plane are deprecated.
/// Your plugin should never need this enumeration.
pub enum HostApplicationId {
    Unknown,
    XPlane,
    PlaneMaker,
    WorldMaker,
    Briefer,
    PartMaker,
    YoungsMod,
    XAuto,
    XAvion,
    ControlPad,
    PFDMap,
    Radar,
}

impl From<i32> for HostApplicationId {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::XPlane,
            2 => Self::PlaneMaker,
            3 => Self::WorldMaker,
            4 => Self::Briefer,
            5 => Self::PlaneMaker,
            6 => Self::YoungsMod,
            7 => Self::XAuto,
            8 => Self::XAvion,
            9 => Self::ControlPad,
            10 => Self::PFDMap,
            11 => Self::Radar,
            _ => Self::Unknown,
        }
    }
}

/// X-Plane and XPLM versions.
pub struct Versions {
    /// Host ID of the app running the plugin.
    pub app_id: HostApplicationId,
    /// X-Plane version.
    pub xplane: i32,
    /// XPLM version.
    pub xplm: i32,
}

/// returns the revision of both X-Plane and the XPLM DLL.
/// In addition returns the host ID of the app running the plugin.
///
/// # Returns
/// Returns [`Versions`].
pub fn get_versions() -> Versions {
    let mut xplane_version = 0;
    let mut xplm_version = 0;
    let mut host_id = 0;
    unsafe { xplm_sys::XPLMGetVersions(&mut xplane_version, &mut xplm_version, &mut host_id) };

    Versions {
        app_id: HostApplicationId::from(host_id),
        xplane: xplane_version,
        xplm: xplm_version,
    }
}

/// Defines what language the sim is running in.
pub enum Language {
    Unknown,
    English,
    French,
    German,
    Italian,
    Spanish,
    Korean,
    Russian,
    Greek,
    Japanese,
    Chinese,
}

impl From<i32> for Language {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::English,
            2 => Self::French,
            3 => Self::German,
            4 => Self::Italian,
            5 => Self::Spanish,
            6 => Self::Korean,
            7 => Self::Russian,
            8 => Self::Greek,
            9 => Self::Japanese,
            10 => Self::Chinese,
            _ => Self::Unknown,
        }
    }
}

/// Returns the [`Language`] the sim is running in.
pub fn get_language() -> Language {
    unsafe { xplm_sys::XPLMGetLanguage() }.into()
}

static ERROR_CALLBACK: OnceLock<fn(&str)> = OnceLock::new();

/// Installs an error-reporting callback for your plugin. Normally the plugin
/// system performs minimum diagnostics to maximize performance.
/// When you install an error callback, you will receive calls due to certain plugin errors,
/// such as passing bad parameters or incorrect data.
///
/// Important: the error callback determines programming errors, e.g. bad API parameters.
/// Every error that is returned by the error callback represents a mistake in your plugin
/// that you should fix. Error callbacks are not used to report expected run-time
/// problems (e.g. disk I/O errors).
///
/// Installing an error callback may activate error checking code that would not normally run,
/// and this may adversely affect performance, so do not leave error callbacks installed in
/// shipping plugins. Since the only useful response to an error is to change code, error
/// callbacks are not useful “in the field”.
///
/// # Arguments
/// * `callback` - a function which accepts `&str` messages.
pub fn set_error_callback(callback: fn(&str)) {
    unsafe extern "C" fn error_callback(message: *const ::std::os::raw::c_char) {
        let message_c = ffi::CStr::from_ptr(message);
        match message_c.to_str() {
            Ok(message_str) => {
                if let Some(handler) = ERROR_CALLBACK.get() {
                    handler(message_str)
                }
            }
            Err(_) => debug_string("Error handler called with an invalid message"),
        }
    }

    ERROR_CALLBACK.get_or_init(|| callback);
    unsafe { xplm_sys::XPLMSetErrorCallback(Some(error_callback)) };
}

/// Outputs a string to the `Log.txt` file. The file is immediately flushed so the data is not lost.
/// This does cause a performance penalty.
///
/// # Arguments
/// * `message` - a message that will be written to the log file.
pub fn debug_string<T: Into<String>>(message: T) {
    if let Ok(message_c) = ffi::CString::new(message.into()) {
        unsafe { xplm_sys::XPLMDebugString(message_c.as_ptr()) };
    }
}
