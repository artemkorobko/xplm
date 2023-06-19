pub mod app;
pub mod error;
pub mod file;
pub mod lang;

use std::{
    ffi,
    ops::{Deref, DerefMut},
    path, str,
    sync::OnceLock,
};

pub use self::error::UtilitiesError;
use self::{
    app::{HostApplicationId, Versions},
    file::DataFileType,
};

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

/// Loads a data file of a given type.
///
/// # Arguments
/// * `file_type` - the type of the file to load. See [`DataFileType`].
/// * `file_path` - the file path that must be relative to the X-System folder.
///
/// # Returns
/// Returns `Ok` in case of success. Otherwise returns
/// * [`UtilitiesError::LoadDataFile`] if data file can't be loaded.
/// * [`UtilitiesError::InvalidDataFilePath`] if file_path contains invalid characters.
pub fn load_data_file<P: AsRef<path::Path>>(file_type: DataFileType, file_path: P) -> Result<()> {
    let file_path_str = file_path
        .as_ref()
        .to_str()
        .ok_or(UtilitiesError::LoadDataFile)?;
    let file_path_c =
        ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    let is_loaded = unsafe {
        xplm_sys::XPLMLoadDataFile(
            file_type as xplm_sys::XPLMDataFileType,
            file_path_c.as_ptr(),
        )
    };

    if is_loaded == 1 {
        Ok(())
    } else {
        Err(UtilitiesError::LoadDataFile)
    }
}

/// Clears the replay. This is only valid with replay movies, not sit files.
///
/// # Returns
/// Returns `Ok` in case of success. Otherwise returns [`UtilitiesError::ClearReplay`].
pub fn clear_replay() -> Result<()> {
    let is_loaded = unsafe {
        xplm_sys::XPLMLoadDataFile(
            DataFileType::ReplayMovie as xplm_sys::XPLMDataFileType,
            std::ptr::null_mut(),
        )
    };

    if is_loaded == 1 {
        Ok(())
    } else {
        Err(UtilitiesError::ClearReplay)
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
/// * [`UtilitiesError::SaveDataFile`] if data file can't be loaded.
/// * [`UtilitiesError::InvalidDataFilePath`] if file_path contains invalid characters.
pub fn save_data_file<P: AsRef<path::Path>>(file_type: DataFileType, file_path: P) -> Result<()> {
    let file_path_str = file_path
        .as_ref()
        .to_str()
        .ok_or(UtilitiesError::SaveDataFile)?;
    let file_path_c =
        ffi::CString::new(file_path_str).map_err(UtilitiesError::InvalidDataFilePath)?;
    let is_saved = unsafe {
        xplm_sys::XPLMSaveDataFile(
            file_type as xplm_sys::XPLMDataFileType,
            file_path_c.as_ptr(),
        )
    };

    if is_saved == 1 {
        Ok(())
    } else {
        Err(UtilitiesError::SaveDataFile)
    }
}

/// Returns the revision of both X-Plane and the XPLM shared libraries.
/// In addition returns the host ID of the app running the plugin.
///
/// # Returns
/// Returns [`Versions`] on success. Otherwise returns [`UtilitiesError`].
pub fn get_versions() -> Result<Versions> {
    let mut xplane_version = 0;
    let mut xplm_version = 0;
    let mut host_id = 0;
    unsafe { xplm_sys::XPLMGetVersions(&mut xplane_version, &mut xplm_version, &mut host_id) };
    Ok(Versions {
        app_id: HostApplicationId::try_from(host_id)?,
        xplane: xplane_version,
        xplm: xplm_version,
    })
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

impl TryFrom<xplm_sys::XPLMLanguageCode> for Language {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMLanguageCode) -> std::result::Result<Self, Self::Error> {
        match value as ::std::os::raw::c_uint {
            xplm_sys::xplm_Language_Unknown => Ok(Self::Unknown),
            xplm_sys::xplm_Language_English => Ok(Self::English),
            xplm_sys::xplm_Language_French => Ok(Self::French),
            xplm_sys::xplm_Language_German => Ok(Self::German),
            xplm_sys::xplm_Language_Italian => Ok(Self::Italian),
            xplm_sys::xplm_Language_Spanish => Ok(Self::Spanish),
            xplm_sys::xplm_Language_Korean => Ok(Self::Korean),
            xplm_sys::xplm_Language_Russian => Ok(Self::Russian),
            xplm_sys::xplm_Language_Greek => Ok(Self::Greek),
            xplm_sys::xplm_Language_Japanese => Ok(Self::Japanese),
            xplm_sys::xplm_Language_Chinese => Ok(Self::Chinese),
            _ => Err(Self::Error::UnknownLanguageCode(value)),
        }
    }
}

/// Returns the [`Language`] the sim is running in.
///
/// # Returns
/// Returns [`Language`] on success. Otherwise returns [`UtilitiesError::UnknownLanguageCode`].
pub fn get_language() -> Result<Language> {
    let code = unsafe { xplm_sys::XPLMGetLanguage() };
    Language::try_from(code)
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
            Err(err) => crate::error!("Error handler called with an invalid message. {}", err),
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
#[repr(u32)]
pub enum VirtualKey {
    Back = xplm_sys::XPLM_VK_BACK,
    Tab = xplm_sys::XPLM_VK_TAB,
    Clear = xplm_sys::XPLM_VK_CLEAR,
    Return = xplm_sys::XPLM_VK_RETURN,
    Escape = xplm_sys::XPLM_VK_ESCAPE,
    Space = xplm_sys::XPLM_VK_SPACE,
    Prior = xplm_sys::XPLM_VK_PRIOR,
    Next = xplm_sys::XPLM_VK_NEXT,
    End = xplm_sys::XPLM_VK_END,
    Home = xplm_sys::XPLM_VK_HOME,
    Left = xplm_sys::XPLM_VK_LEFT,
    Up = xplm_sys::XPLM_VK_UP,
    Right = xplm_sys::XPLM_VK_RIGHT,
    Down = xplm_sys::XPLM_VK_DOWN,
    Select = xplm_sys::XPLM_VK_SELECT,
    Print = xplm_sys::XPLM_VK_PRINT,
    Execute = xplm_sys::XPLM_VK_EXECUTE,
    Snapshot = xplm_sys::XPLM_VK_SNAPSHOT,
    Insert = xplm_sys::XPLM_VK_INSERT,
    Delete = xplm_sys::XPLM_VK_DELETE,
    Help = xplm_sys::XPLM_VK_HELP,
    Zero = xplm_sys::XPLM_VK_0,
    One = xplm_sys::XPLM_VK_1,
    Two = xplm_sys::XPLM_VK_2,
    Three = xplm_sys::XPLM_VK_3,
    Four = xplm_sys::XPLM_VK_4,
    Five = xplm_sys::XPLM_VK_5,
    Six = xplm_sys::XPLM_VK_6,
    Seven = xplm_sys::XPLM_VK_7,
    Eight = xplm_sys::XPLM_VK_8,
    Nine = xplm_sys::XPLM_VK_9,
    A = xplm_sys::XPLM_VK_A,
    B = xplm_sys::XPLM_VK_B,
    C = xplm_sys::XPLM_VK_C,
    D = xplm_sys::XPLM_VK_D,
    E = xplm_sys::XPLM_VK_E,
    F = xplm_sys::XPLM_VK_F,
    G = xplm_sys::XPLM_VK_G,
    H = xplm_sys::XPLM_VK_H,
    I = xplm_sys::XPLM_VK_I,
    J = xplm_sys::XPLM_VK_J,
    K = xplm_sys::XPLM_VK_K,
    L = xplm_sys::XPLM_VK_L,
    M = xplm_sys::XPLM_VK_M,
    N = xplm_sys::XPLM_VK_N,
    O = xplm_sys::XPLM_VK_O,
    P = xplm_sys::XPLM_VK_P,
    Q = xplm_sys::XPLM_VK_Q,
    R = xplm_sys::XPLM_VK_R,
    S = xplm_sys::XPLM_VK_S,
    T = xplm_sys::XPLM_VK_T,
    U = xplm_sys::XPLM_VK_U,
    V = xplm_sys::XPLM_VK_V,
    W = xplm_sys::XPLM_VK_W,
    X = xplm_sys::XPLM_VK_X,
    Y = xplm_sys::XPLM_VK_Y,
    Z = xplm_sys::XPLM_VK_Z,
    Numpad0 = xplm_sys::XPLM_VK_NUMPAD0,
    Numpad1 = xplm_sys::XPLM_VK_NUMPAD1,
    Numpad2 = xplm_sys::XPLM_VK_NUMPAD2,
    Numpad3 = xplm_sys::XPLM_VK_NUMPAD3,
    Numpad4 = xplm_sys::XPLM_VK_NUMPAD4,
    Numpad5 = xplm_sys::XPLM_VK_NUMPAD5,
    Numpad6 = xplm_sys::XPLM_VK_NUMPAD6,
    Numpad7 = xplm_sys::XPLM_VK_NUMPAD7,
    Numpad8 = xplm_sys::XPLM_VK_NUMPAD8,
    Numpad9 = xplm_sys::XPLM_VK_NUMPAD9,
    Multiply = xplm_sys::XPLM_VK_MULTIPLY,
    Add = xplm_sys::XPLM_VK_ADD,
    Separator = xplm_sys::XPLM_VK_SEPARATOR,
    Subtract = xplm_sys::XPLM_VK_SUBTRACT,
    Decimal = xplm_sys::XPLM_VK_DECIMAL,
    Divide = xplm_sys::XPLM_VK_DIVIDE,
    F1 = xplm_sys::XPLM_VK_F1,
    F2 = xplm_sys::XPLM_VK_F2,
    F3 = xplm_sys::XPLM_VK_F3,
    F4 = xplm_sys::XPLM_VK_F4,
    F5 = xplm_sys::XPLM_VK_F5,
    F6 = xplm_sys::XPLM_VK_F6,
    F7 = xplm_sys::XPLM_VK_F7,
    F8 = xplm_sys::XPLM_VK_F8,
    F9 = xplm_sys::XPLM_VK_F9,
    F10 = xplm_sys::XPLM_VK_F10,
    F11 = xplm_sys::XPLM_VK_F11,
    F12 = xplm_sys::XPLM_VK_F12,
    F13 = xplm_sys::XPLM_VK_F13,
    F14 = xplm_sys::XPLM_VK_F14,
    F15 = xplm_sys::XPLM_VK_F15,
    F16 = xplm_sys::XPLM_VK_F16,
    F17 = xplm_sys::XPLM_VK_F17,
    F18 = xplm_sys::XPLM_VK_F18,
    F19 = xplm_sys::XPLM_VK_F19,
    F20 = xplm_sys::XPLM_VK_F20,
    F21 = xplm_sys::XPLM_VK_F21,
    F22 = xplm_sys::XPLM_VK_F22,
    F23 = xplm_sys::XPLM_VK_F23,
    F24 = xplm_sys::XPLM_VK_F24,
    Equal = xplm_sys::XPLM_VK_EQUAL,
    Minus = xplm_sys::XPLM_VK_MINUS,
    RBrace = xplm_sys::XPLM_VK_RBRACE,
    LBrace = xplm_sys::XPLM_VK_LBRACE,
    Quote = xplm_sys::XPLM_VK_QUOTE,
    Semicolon = xplm_sys::XPLM_VK_SEMICOLON,
    Backslash = xplm_sys::XPLM_VK_BACKSLASH,
    Comma = xplm_sys::XPLM_VK_COMMA,
    Slash = xplm_sys::XPLM_VK_SLASH,
    Period = xplm_sys::XPLM_VK_PERIOD,
    Backquote = xplm_sys::XPLM_VK_BACKQUOTE,
    Enter = xplm_sys::XPLM_VK_ENTER,
    NumpadEnter = xplm_sys::XPLM_VK_NUMPAD_ENT,
    NumpadEq = xplm_sys::XPLM_VK_NUMPAD_EQ,
}

impl TryFrom<::std::os::raw::c_char> for VirtualKey {
    type Error = UtilitiesError;

    fn try_from(value: ::std::os::raw::c_char) -> std::result::Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::XPLM_VK_BACK => Ok(Self::Back),
            xplm_sys::XPLM_VK_TAB => Ok(Self::Tab),
            xplm_sys::XPLM_VK_CLEAR => Ok(Self::Clear),
            xplm_sys::XPLM_VK_RETURN => Ok(Self::Return),
            xplm_sys::XPLM_VK_ESCAPE => Ok(Self::Escape),
            xplm_sys::XPLM_VK_SPACE => Ok(Self::Space),
            xplm_sys::XPLM_VK_PRIOR => Ok(Self::Prior),
            xplm_sys::XPLM_VK_NEXT => Ok(Self::Next),
            xplm_sys::XPLM_VK_END => Ok(Self::End),
            xplm_sys::XPLM_VK_HOME => Ok(Self::Home),
            xplm_sys::XPLM_VK_LEFT => Ok(Self::Left),
            xplm_sys::XPLM_VK_UP => Ok(Self::Up),
            xplm_sys::XPLM_VK_RIGHT => Ok(Self::Right),
            xplm_sys::XPLM_VK_DOWN => Ok(Self::Down),
            xplm_sys::XPLM_VK_SELECT => Ok(Self::Select),
            xplm_sys::XPLM_VK_PRINT => Ok(Self::Print),
            xplm_sys::XPLM_VK_EXECUTE => Ok(Self::Execute),
            xplm_sys::XPLM_VK_SNAPSHOT => Ok(Self::Snapshot),
            xplm_sys::XPLM_VK_INSERT => Ok(Self::Insert),
            xplm_sys::XPLM_VK_DELETE => Ok(Self::Delete),
            xplm_sys::XPLM_VK_HELP => Ok(Self::Help),
            xplm_sys::XPLM_VK_0 => Ok(Self::Zero),
            xplm_sys::XPLM_VK_1 => Ok(Self::One),
            xplm_sys::XPLM_VK_2 => Ok(Self::Two),
            xplm_sys::XPLM_VK_3 => Ok(Self::Three),
            xplm_sys::XPLM_VK_4 => Ok(Self::Four),
            xplm_sys::XPLM_VK_5 => Ok(Self::Five),
            xplm_sys::XPLM_VK_6 => Ok(Self::Six),
            xplm_sys::XPLM_VK_7 => Ok(Self::Seven),
            xplm_sys::XPLM_VK_8 => Ok(Self::Eight),
            xplm_sys::XPLM_VK_9 => Ok(Self::Nine),
            xplm_sys::XPLM_VK_A => Ok(Self::A),
            xplm_sys::XPLM_VK_B => Ok(Self::B),
            xplm_sys::XPLM_VK_C => Ok(Self::C),
            xplm_sys::XPLM_VK_D => Ok(Self::D),
            xplm_sys::XPLM_VK_E => Ok(Self::E),
            xplm_sys::XPLM_VK_F => Ok(Self::F),
            xplm_sys::XPLM_VK_G => Ok(Self::G),
            xplm_sys::XPLM_VK_H => Ok(Self::H),
            xplm_sys::XPLM_VK_I => Ok(Self::I),
            xplm_sys::XPLM_VK_J => Ok(Self::J),
            xplm_sys::XPLM_VK_K => Ok(Self::K),
            xplm_sys::XPLM_VK_L => Ok(Self::L),
            xplm_sys::XPLM_VK_M => Ok(Self::M),
            xplm_sys::XPLM_VK_N => Ok(Self::N),
            xplm_sys::XPLM_VK_O => Ok(Self::O),
            xplm_sys::XPLM_VK_P => Ok(Self::P),
            xplm_sys::XPLM_VK_Q => Ok(Self::Q),
            xplm_sys::XPLM_VK_R => Ok(Self::R),
            xplm_sys::XPLM_VK_S => Ok(Self::S),
            xplm_sys::XPLM_VK_T => Ok(Self::T),
            xplm_sys::XPLM_VK_U => Ok(Self::U),
            xplm_sys::XPLM_VK_V => Ok(Self::V),
            xplm_sys::XPLM_VK_W => Ok(Self::W),
            xplm_sys::XPLM_VK_X => Ok(Self::X),
            xplm_sys::XPLM_VK_Y => Ok(Self::Y),
            xplm_sys::XPLM_VK_Z => Ok(Self::Z),
            xplm_sys::XPLM_VK_NUMPAD0 => Ok(Self::Numpad0),
            xplm_sys::XPLM_VK_NUMPAD1 => Ok(Self::Numpad1),
            xplm_sys::XPLM_VK_NUMPAD2 => Ok(Self::Numpad2),
            xplm_sys::XPLM_VK_NUMPAD3 => Ok(Self::Numpad3),
            xplm_sys::XPLM_VK_NUMPAD4 => Ok(Self::Numpad4),
            xplm_sys::XPLM_VK_NUMPAD5 => Ok(Self::Numpad5),
            xplm_sys::XPLM_VK_NUMPAD6 => Ok(Self::Numpad6),
            xplm_sys::XPLM_VK_NUMPAD7 => Ok(Self::Numpad7),
            xplm_sys::XPLM_VK_NUMPAD8 => Ok(Self::Numpad8),
            xplm_sys::XPLM_VK_NUMPAD9 => Ok(Self::Numpad9),
            xplm_sys::XPLM_VK_MULTIPLY => Ok(Self::Multiply),
            xplm_sys::XPLM_VK_ADD => Ok(Self::Add),
            xplm_sys::XPLM_VK_SEPARATOR => Ok(Self::Separator),
            xplm_sys::XPLM_VK_SUBTRACT => Ok(Self::Subtract),
            xplm_sys::XPLM_VK_DECIMAL => Ok(Self::Decimal),
            xplm_sys::XPLM_VK_DIVIDE => Ok(Self::Divide),
            xplm_sys::XPLM_VK_F1 => Ok(Self::F1),
            xplm_sys::XPLM_VK_F2 => Ok(Self::F2),
            xplm_sys::XPLM_VK_F3 => Ok(Self::F3),
            xplm_sys::XPLM_VK_F4 => Ok(Self::F4),
            xplm_sys::XPLM_VK_F5 => Ok(Self::F5),
            xplm_sys::XPLM_VK_F6 => Ok(Self::F6),
            xplm_sys::XPLM_VK_F7 => Ok(Self::F7),
            xplm_sys::XPLM_VK_F8 => Ok(Self::F8),
            xplm_sys::XPLM_VK_F9 => Ok(Self::F9),
            xplm_sys::XPLM_VK_F10 => Ok(Self::F10),
            xplm_sys::XPLM_VK_F11 => Ok(Self::F11),
            xplm_sys::XPLM_VK_F12 => Ok(Self::F12),
            xplm_sys::XPLM_VK_F13 => Ok(Self::F13),
            xplm_sys::XPLM_VK_F14 => Ok(Self::F14),
            xplm_sys::XPLM_VK_F15 => Ok(Self::F15),
            xplm_sys::XPLM_VK_F16 => Ok(Self::F16),
            xplm_sys::XPLM_VK_F17 => Ok(Self::F17),
            xplm_sys::XPLM_VK_F18 => Ok(Self::F18),
            xplm_sys::XPLM_VK_F19 => Ok(Self::F19),
            xplm_sys::XPLM_VK_F20 => Ok(Self::F20),
            xplm_sys::XPLM_VK_F21 => Ok(Self::F21),
            xplm_sys::XPLM_VK_F22 => Ok(Self::F22),
            xplm_sys::XPLM_VK_F23 => Ok(Self::F23),
            xplm_sys::XPLM_VK_F24 => Ok(Self::F24),
            xplm_sys::XPLM_VK_EQUAL => Ok(Self::Equal),
            xplm_sys::XPLM_VK_MINUS => Ok(Self::Minus),
            xplm_sys::XPLM_VK_RBRACE => Ok(Self::RBrace),
            xplm_sys::XPLM_VK_LBRACE => Ok(Self::LBrace),
            xplm_sys::XPLM_VK_QUOTE => Ok(Self::Quote),
            xplm_sys::XPLM_VK_SEMICOLON => Ok(Self::Semicolon),
            xplm_sys::XPLM_VK_BACKSLASH => Ok(Self::Backslash),
            xplm_sys::XPLM_VK_COMMA => Ok(Self::Comma),
            xplm_sys::XPLM_VK_SLASH => Ok(Self::Slash),
            xplm_sys::XPLM_VK_PERIOD => Ok(Self::Period),
            xplm_sys::XPLM_VK_BACKQUOTE => Ok(Self::Backquote),
            xplm_sys::XPLM_VK_ENTER => Ok(Self::Enter),
            xplm_sys::XPLM_VK_NUMPAD_ENT => Ok(Self::NumpadEnter),
            xplm_sys::XPLM_VK_NUMPAD_EQ => Ok(Self::NumpadEq),
            _ => Err(Self::Error::InvalidVirtualKey(value)),
        }
    }
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
    .map_err(UtilitiesError::InvalidVirtualKeyDescription)
}

/// Reloads the current set of scenery.
pub fn reload_scenery() {
    unsafe { xplm_sys::XPLMReloadScenery() };
}

/// An opaque identifier for an X-Plane command
pub struct Command(xplm_sys::XPLMCommandRef);

impl TryFrom<xplm_sys::XPLMCommandRef> for Command {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMCommandRef) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidCommand)
        } else {
            Ok(Command(value))
        }
    }
}

impl Deref for Command {
    type Target = xplm_sys::XPLMCommandRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Looks up a command by name.
///
/// # Arguments
/// * `name` - a command name.
///
/// # Returns
/// Returns [`Command`] on success. Otherwise returns:
/// - [`None`] in case command does not exists.
/// - [`UtilitiesError::InvalidCommandName`] in case of malformed command name.
pub fn find_command<T: Into<String>>(name: T) -> Result<Option<Command>> {
    let name_c = ffi::CString::new(name.into()).map_err(UtilitiesError::InvalidCommandName)?;
    let command = unsafe { xplm_sys::XPLMFindCommand(name_c.as_ptr()) };
    if command.is_null() {
        Ok(None)
    } else {
        Ok(Command::try_from(command).ok())
    }
}

/// Starts the execution of a command.
///
/// # Arguments
/// * `command` - the [`Command`] to begin execution of.
pub fn command_begin(command: &Command) {
    unsafe { xplm_sys::XPLMCommandBegin(command.0) };
}

/// Ends the execution of a given command that was previously started.
///
/// # Arguments
/// * `command` - the [`Command`] to end execution of.
pub fn command_end(command: &Command) {
    unsafe { xplm_sys::XPLMCommandEnd(command.0) };
}

/// Executes a given command momentarily, that is, the command begins and ends immediately.
///
/// # Arguments
/// * `command` - the [`Command`] to execute.
pub fn command_once(command: &Command) {
    unsafe { xplm_sys::XPLMCommandOnce(command.0) };
}

/// Creates a new command for a given name and description.
///
/// # Arguments
/// * `name` - a command name.
/// * `description` - a command description.
///
/// # Returns
/// Returns [`Command`] in case of success.
/// - [`UtilitiesError::InvalidCommandName`] in case of malformed name argument.
/// - [`UtilitiesError::InvalidCommandDescription`] in case of malformed description argument.
pub fn create_command<N, D>(name: N, description: D) -> Result<Command>
where
    N: Into<String>,
    D: Into<String>,
{
    let name_c = ffi::CString::new(name.into()).map_err(UtilitiesError::InvalidCommandName)?;
    let description_c =
        ffi::CString::new(description.into()).map_err(UtilitiesError::InvalidCommandDescription)?;
    let command = unsafe { xplm_sys::XPLMCreateCommand(name_c.as_ptr(), description_c.as_ptr()) };
    Command::try_from(command)
}

/// Command handler.
pub trait CommandHandler: 'static {
    /// Called when the command begins (corresponds to a button being pressed down)
    fn command_begin(&mut self);
    /// Called frequently while the command button is held down
    fn command_continue(&mut self);
    /// Called when the command ends (corresponds to a button being released)
    fn command_end(&mut self);
}

/// A link to [`CommandHandler`] for a given [`Command`].
pub struct CommandLink {
    command: xplm_sys::XPLMCommandRef,
    handler: Box<dyn CommandHandler>,
}

/// A command handler record to keep a registration alive.
pub struct CommandHandlerRecord {
    link: Box<CommandLink>,
    execution_time: CommandExecutionTime,
}

impl Drop for CommandHandlerRecord {
    fn drop(&mut self) {
        unregister_command_handler(self);
    }
}

/// A command execution time.
#[derive(Copy, Clone)]
pub enum CommandExecutionTime {
    /// A callback will run before X-Plane.
    BeforeXPlane = 1,
    /// A callback will run after X-Plane.
    AfterXPlane = 0,
}

/// Registers a callback to be called when a command is executed.
///
/// # Arguments
/// * `command` - the command to attach the handler to.
/// * `execution_time` - the time when handler should be executed. See [`CommandExecutionTime`].
/// * `handler` - the handler which handles command execution. See [`CommandHandler`].
///
/// # Returns
/// Returns a [`CommandHandlerRecord`] which should be kept util execution is needed.
/// Dropping this record will unregister the handler.
pub fn register_command_handler<H: CommandHandler>(
    command: &Command,
    execution_time: CommandExecutionTime,
    handler: H,
) -> CommandHandlerRecord {
    let mut link = Box::new(CommandLink {
        command: command.0,
        handler: Box::new(handler),
    });

    let link_ptr: *mut CommandLink = link.deref_mut();

    unsafe {
        xplm_sys::XPLMRegisterCommandHandler(
            command.0,
            Some(command_handler),
            execution_time as _,
            link_ptr as *mut _,
        )
    };

    CommandHandlerRecord {
        link,
        execution_time,
    }
}

unsafe extern "C" fn command_handler(
    command: xplm_sys::XPLMCommandRef,
    phase: xplm_sys::XPLMCommandPhase,
    refcon: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    const CONTINUE_EXECUTION: ::std::os::raw::c_int = 1;
    const TERMINATE_EXECUTION: ::std::os::raw::c_int = 1;
    let link = refcon as *mut CommandLink;
    if (*link).command == command {
        let handler = (*link).handler.deref_mut();
        match phase as ::std::os::raw::c_uint {
            xplm_sys::xplm_CommandBegin => (*handler).command_begin(),
            xplm_sys::xplm_CommandContinue => (*handler).command_continue(),
            xplm_sys::xplm_CommandEnd => (*handler).command_end(),
            _ => {}
        };
        TERMINATE_EXECUTION
    } else {
        CONTINUE_EXECUTION
    }
}

/// Removes a command callback registered with [`register_command_handler`] API call.
pub fn unregister_command_handler(record: &mut CommandHandlerRecord) {
    let link_ptr: *mut CommandLink = record.link.deref_mut();

    unsafe {
        xplm_sys::XPLMUnregisterCommandHandler(
            record.link.command,
            Some(command_handler),
            record.execution_time as ::std::os::raw::c_int,
            link_ptr as *mut ::std::os::raw::c_void,
        )
    };
}
