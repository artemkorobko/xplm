pub mod app;
pub mod command;
pub mod error;
pub mod file;
pub mod key;
pub mod lang;

use std::ops::Deref;
use std::{ffi, ops::DerefMut, path, str, sync::OnceLock};

pub use self::app::{HostApplicationId, Versions};
pub use self::command::Command;
pub use self::command::{CommandExecutionTime, CommandHandler, CommandHandlerRecord, CommandLink};
pub use self::error::UtilitiesError;
pub use self::file::DataFileType;
pub use self::key::VirtualKey;
pub use self::lang::Language;

pub type Result<T> = std::result::Result<T, UtilitiesError>;

/// Returns the full path to the X-System folder. Note that this is a directory path,
/// so it ends in a trailing `:` or `/`.
///
/// # Returns
/// Returns system path on success. Otherwise returns [`UtilitiesError`].
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
/// Returns preferences file path on success. Otherwise returns [`UtilitiesError`].
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
/// Returns directory separator on success. Otherwise returns [`UtilitiesError`].
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

/// Returns the [`Language`] the sim is running in.
///
/// # Returns
/// Returns [`Language`] on success. Otherwise returns [`UtilitiesError`].
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

/// Looks up a command by name.
///
/// # Arguments
/// * `name` - a command name.
///
/// # Returns
/// Returns [`Command`] on success. Otherwise returns:
/// - [`None`] in case command does not exists.
/// - [`UtilitiesError`] in case of malformed command name.
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
    unsafe { xplm_sys::XPLMCommandBegin(*command.deref()) };
}

/// Ends the execution of a given command that was previously started.
///
/// # Arguments
/// * `command` - the [`Command`] to end execution of.
pub fn command_end(command: &Command) {
    unsafe { xplm_sys::XPLMCommandEnd(*command.deref()) };
}

/// Executes a given command momentarily, that is, the command begins and ends immediately.
///
/// # Arguments
/// * `command` - the [`Command`] to execute.
pub fn command_once(command: &Command) {
    unsafe { xplm_sys::XPLMCommandOnce(*command.deref()) };
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
        command: *command.deref(),
        handler: Box::new(handler),
    });

    let link_ptr: *mut CommandLink = link.deref_mut();

    unsafe {
        xplm_sys::XPLMRegisterCommandHandler(
            *command.deref(),
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
    if (*link).links_with(command) {
        match phase as ::std::os::raw::c_uint {
            xplm_sys::xplm_CommandBegin => (*link).command_begin(),
            xplm_sys::xplm_CommandContinue => (*link).command_continue(),
            xplm_sys::xplm_CommandEnd => (*link).command_end(),
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
            record.execution_time.into(),
            link_ptr as *mut ::std::os::raw::c_void,
        )
    };
}
