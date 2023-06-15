use std::{ffi, path, str, sync::OnceLock};

use thiserror::Error;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
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
    LoadDataFileError,
    /// Unable to save data file
    #[error("unable to save data file")]
    SaveDataFileError,
    /// Invalid virtual key description string returned from X-Plane
    #[error("invalid virtual key description {0}")]
    InvalidVKDescription(ffi::IntoStringError),
}

pub type Result<T> = std::result::Result<T, UtilitiesError>;

/// Returns the full path to the X-System folder. Note that this is a directory path,
/// so it ends in a trailing `:` or `/`.
///
/// # Returns
/// Returns system path on success. Otherwise returns [`UtilitiesError::InvalidSystemPath`].
pub fn get_system_path() -> Result<path::PathBuf> {
    unsafe {
        let mut buf = [0; 4096];
        xplm_sys::XPLMGetSystemPath(buf.as_mut_ptr());
        ffi::CStr::from_ptr(buf.as_ptr()).to_owned().into_string()
    }
    .map(|path| path::PathBuf::from(&path))
    .map_err(UtilitiesError::InvalidPrefsPath)
}

/// Returns a full path to a file that is within X-Plane’s preferences directory.
///
/// # Returns
/// Returns preferences file path on success. Otherwise returns [`UtilitiesError::InvalidPrefsPath`].
pub fn get_prefs_path() -> Result<path::PathBuf> {
    unsafe {
        let mut buf = [0; 4096];
        xplm_sys::XPLMGetPrefsPath(buf.as_mut_ptr());
        ffi::CStr::from_ptr(buf.as_ptr()).to_owned().into_string()
    }
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
    let file_path_c =
        ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    let is_loaded =
        unsafe { xplm_sys::XPLMLoadDataFile(file_type as i32, file_path_c.as_ptr()) == 1 };
    if is_loaded {
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
    let file_type = DataFileType::ReplayMovie as i32;
    let is_loaded = unsafe { xplm_sys::XPLMLoadDataFile(file_type, std::ptr::null_mut()) == 1 };
    if is_loaded {
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
    let file_path_c =
        ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    let is_saved =
        unsafe { xplm_sys::XPLMSaveDataFile(file_type as i32, file_path_c.as_ptr()) == 1 };
    if is_saved {
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

/// Displays the string in a translucent overlay over the current display and also speaks the string
/// if text-to-speech is enabled. The string is spoken asynchronously, this function returns immediately.
/// This function may not speak or print depending on user preferences.
///
/// # Arguments
/// * `message` - a message that will be spoken.
pub fn speak_string<T: Into<String>>(message: T) {
    if let Ok(message_c) = ffi::CString::new(message.into()) {
        unsafe { xplm_sys::XPLMSpeakString(message_c.as_ptr()) };
    }
}

/// A cross-platform virtual key codes for every distinct keyboard press on the computer.
#[repr(u8)]
pub enum VirtualKey {
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Escape = 0x1B,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    Zero = 0x30,
    One = 0x31,
    Two = 0x32,
    Three = 0x33,
    Four = 0x34,
    Five = 0x35,
    Six = 0x36,
    Seven = 0x37,
    Eight = 0x38,
    Nine = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    Multiply = 0x6A,
    Add = 0x6B,
    Separator = 0x6C,
    Subtract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    Equal = 0xB0,
    Minus = 0xB1,
    RBrace = 0xB2,
    LBrace = 0xB3,
    Quote = 0xB4,
    Semicolon = 0xB5,
    Backslash = 0xB6,
    Comma = 0xB7,
    Slash = 0xB8,
    Period = 0xB9,
    Backquote = 0xBA,
    Enter = 0xBB,
    NumpadEnter = 0xBC,
    NumpadEq = 0xBD,
}

/// Returns a human-readable string describing the character.
///
/// # Arguments
/// * `key` - a [`VirtualKey`] code.
pub fn get_virtual_key_description(key: VirtualKey) -> Result<Option<String>> {
    unsafe {
        let opcode = key as ::std::os::raw::c_char;
        let description_c = xplm_sys::XPLMGetVirtualKeyDescription(opcode);
        if description_c.is_null() {
            Ok(None)
        } else {
            ffi::CStr::from_ptr(description_c)
                .to_owned()
                .into_string()
                .map(Some)
        }
    }
    .map_err(UtilitiesError::InvalidVKDescription)
}
