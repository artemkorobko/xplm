use std::{ffi, string};

use thiserror::Error;

use super::ffi::FromStringBuf;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
pub enum PluginError {
    /// Invalid plugin ID
    #[error("invalid plugin id: {0}")]
    InvalidId(xplm_sys::XPLMPluginID),
    /// Invalid output string passed from Rust to C
    #[error("invalid output string {0}")]
    InvalidOutputString(ffi::NulError),
    /// Invalid input string passed from C to Rust
    #[error("invalid input string {0}")]
    InvalidInputString(string::FromUtf8Error),
}

impl From<ffi::NulError> for PluginError {
    fn from(value: ffi::NulError) -> Self {
        Self::InvalidOutputString(value)
    }
}

impl From<string::FromUtf8Error> for PluginError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::InvalidInputString(value)
    }
}

/// A plugin identifier
#[derive(Copy, Clone, Debug)]
pub struct PluginId(xplm_sys::XPLMPluginID);

impl TryFrom<xplm_sys::XPLMPluginID> for PluginId {
    type Error = PluginError;

    fn try_from(value: xplm_sys::XPLMPluginID) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(Self::Error::InvalidId(value))
        } else {
            Ok(PluginId(value))
        }
    }
}

/// Returns the plugin ID of the calling plug-in. Call this to get your own ID.
///
/// # Returns
/// Returns [`PluginId`] in case of success. Otherwise returns [`PluginError::InvalidId`].
pub fn get_my_id() -> Result<PluginId, PluginError> {
    let id = unsafe { xplm_sys::XPLMGetMyID() };
    PluginId::try_from(id)
}

/// Returns the total number of plug-ins that are loaded, both disabled and enabled.
pub fn count_plugins() -> usize {
    unsafe { xplm_sys::XPLMCountPlugins() as usize }
}

/// Returns the ID of a plug-in by index. Plugins may be returned in any arbitrary order.
///
/// # Arguments
/// * `index` - 0 based index from 0 to [`count_plugins`]-1, inclusive.
///
/// # Returns
/// Returns [`PluginId`] in case of success. Otherwise returns [`PluginError::InvalidId`].
pub fn get_nth_plugin(index: usize) -> Result<PluginId, PluginError> {
    let id = unsafe { xplm_sys::XPLMGetNthPlugin(index as i32) };
    PluginId::try_from(id)
}

/// Returns the plug-in ID of the plug-in whose file exists at the passed in absolute
/// file system path.
///
/// # Arguments
/// * `path` - absolute file system path.
///
/// # Returns
/// Returns [`PluginId`] in case of success. Otherwise returns
/// * [`PluginError::InvalidId`] if the path does not point to a currently loaded plug-in.
/// * [`PluginError::InvalidOutputString`] if path contains invalid characters.
pub fn find_plugin_by_path(path: &str) -> Result<PluginId, PluginError> {
    let c_string = ffi::CString::new(path)?;
    let id = unsafe { xplm_sys::XPLMFindPluginByPath(c_string.as_ptr()) };
    PluginId::try_from(id)
}

/// Returns the plug-in ID of the plug-in whose signature matches what is
/// passed in. Signatures are the best way to identify another plug-in as
/// they are independent of the file system path of a plug-in or the
/// human-readable plug-in name, and should be unique for all plug-ins.
/// Use this routine to locate another plugin that your plugin interoperates with.
///
/// # Arguments
/// * `signature` - the signature of the plug-in.
///
/// # Returns
/// Returns [`PluginId`] in case of success. Otherwise returns
/// * [`PluginError::InvalidId`] if the path does not point to a currently loaded plug-in.
/// * [`PluginError::InvalidOutputString`] if path contains invalid characters.
pub fn find_plugin_by_signature(signature: &str) -> Result<PluginId, PluginError> {
    let c_string = ffi::CString::new(signature)?;
    let id = unsafe { xplm_sys::XPLMFindPluginBySignature(c_string.as_ptr()) };
    PluginId::try_from(id)
}

/// A plugin info.
pub struct PluginInfo {
    /// A plugin name.
    pub name: String,
    /// An absolute file system path.
    pub file_path: String,
    /// A plugin signature.
    pub signature: String,
    /// A plugin description.
    pub description: String,
}

/// Returns information about a plug-in.
///
/// # Arguments
/// * `id` - the plugin identifier. See [`PluginId`].
///
/// # Returns
/// Returns [`PluginInfo`] in case of success.
/// Otherwise returns [`PluginError::InvalidInputString`] if at leat one of
/// the [`PluginInfo`] fields contains invalid character.
pub fn get_plugin_info(id: &PluginId) -> Result<PluginInfo, PluginError> {
    const BUF_LEN: usize = 256;
    let mut out_name = [0; BUF_LEN];
    let mut out_file_path = [0; BUF_LEN];
    let mut out_signature = [0; BUF_LEN];
    let mut out_description = [0; BUF_LEN];

    unsafe {
        xplm_sys::XPLMGetPluginInfo(
            id.0,
            out_name.as_mut_ptr(),
            out_file_path.as_mut_ptr(),
            out_signature.as_mut_ptr(),
            out_description.as_mut_ptr(),
        )
    };

    Ok(PluginInfo {
        name: String::from_string_buf(out_name)?,
        file_path: String::from_string_buf(out_file_path)?,
        signature: String::from_string_buf(out_signature)?,
        description: String::from_string_buf(out_description)?,
    })
}

/// Returns whether the specified plug-in is enabled for running.
///
/// # Arguments
/// * `id` - the plugin identifier. See [`PluginId`].
pub fn is_plugin_enabled(id: &PluginId) -> bool {
    unsafe { xplm_sys::XPLMIsPluginEnabled(id.0) == 1 }
}

/// Enables a plug-in if it is not already enabled. Plugins may fail to enable
/// (for example, if resources cannot be acquired) by returning 0 from their XPluginEnable callback.
///
/// # Arguments
/// * `id` - the plugin identifier. See [`PluginId`].
///
/// # Returns
/// Returns `true` if the plugin was enabled or successfully enables itself. Otherwise returns `false`.
///
pub fn enable_plugin(id: &PluginId) -> bool {
    unsafe { xplm_sys::XPLMEnablePlugin(id.0) == 1 }
}

/// Disables an enabled plug-in.
pub fn disable_plugin(id: &PluginId) {
    unsafe { xplm_sys::XPLMDisablePlugin(id.0) };
}

/// Reloads all plug-ins. Once this routine is called and you return from the callback
/// you were within (e.g. a menu select callback) you will receive your XPluginDisable
/// and XPluginStop callbacks and your DLL will be unloaded, then the start process happens
/// as if the sim was starting up.
pub fn reload_plugins() {
    unsafe { xplm_sys::XPLMReloadPlugins() };
}

/// A trait which declares convertion to message parameter.
pub trait AsMessageParam {
    /// Return the memory pointer to the message parameter.
    fn as_message_param(&self) -> *mut ::std::os::raw::c_void;
}

/// A message parameter that gets ignored when sending messages.
pub struct NoMessageParam;

impl AsMessageParam for NoMessageParam {
    fn as_message_param(&self) -> *mut std::os::raw::c_void {
        std::ptr::null_mut()
    }
}

/// Sends a message to another plug-in or X-Plane. Only enabled plug-ins with a message
/// receive function receive the message.
///
/// # Arguments
/// * `id` - the plugin identifier. See [`PluginId`].
/// * `message` - the unique message identifier.
pub fn send_message_to_plugin<P: AsMessageParam>(id: &PluginId, message: i32, param: P) {
    unsafe { xplm_sys::XPLMSendMessageToPlugin(id.0, message, param.as_message_param()) };
}

/// Broadcasts a message to all plug-ins. Only enabled plug-ins with a message
/// receive function receive the message.
///
/// # Arguments
/// * `message` - the unique message identifier.
pub fn send_message_to_all_plugins<P: AsMessageParam>(message: i32, param: P) {
    const XPLM_NO_PLUGIN_ID: i32 = -1;
    unsafe {
        xplm_sys::XPLMSendMessageToPlugin(XPLM_NO_PLUGIN_ID, message, param.as_message_param())
    };
}

/// Plugin advanced features.
pub enum Feature {
    /// Causes plugin to receive drawing hook callbacks when X-Plane builds its off-screen
    /// reflection and shadow rendering passes.
    WantsReflections,
    /// OS X paths will match the native OS X Unix. Windows will use forward slashes but
    /// preserve C:\ or another drive letter when using complete file paths.
    UseNativePaths,
    /// Tells the widgets library to use new, modern X-Plane backed XPLMDisplay windows
    /// to anchor all widget trees. Without it, widgets will always use legacy windows.
    UseNativeWidgetsWindows,
    /// Tells X-Plane to to send the enabling plugin the new XPLM_MSG_DATAREFS_ADDED message
    /// any time new datarefs are added. The SDK will coalesce consecutive dataref registrations
    /// to minimize the number of messages sent.
    WantsDatarefNotifications,
}

impl Feature {
    pub fn name(&self) -> &'static str {
        match self {
            Feature::WantsReflections => "XPLM_WANTS_REFLECTIONS",
            Feature::UseNativePaths => "XPLM_USE_NATIVE_PATHS",
            Feature::UseNativeWidgetsWindows => "XPLM_USE_NATIVE_WIDGET_WINDOWS",
            Feature::WantsDatarefNotifications => "XPLM_WANTS_DATAREF_NOTIFICATIONS",
        }
    }
}

/// Checks wether the given feature exists.
///
/// # Arguments
/// * `feature` - the feature to check.
///
/// # Returns
/// Returns `true` if the given installation of X-Plane supports a feature. Otherwise returns `false`.
pub fn has_feature(feature: Feature) -> bool {
    if let Ok(name) = ffi::CString::new(feature.name()) {
        unsafe { xplm_sys::XPLMHasFeature(name.as_ptr()) == 1 }
    } else {
        false
    }
}

/// Checks wether the given feature enabld.
///
/// # Arguments
/// * `feature` - the feature to check.
///
/// # Returns
/// Returns `true` if the given feature is currently enabled for plugin. Otherwise returns `false`.
pub fn is_feature_enabled(feature: Feature) -> bool {
    if let Ok(name) = ffi::CString::new(feature.name()) {
        unsafe { xplm_sys::XPLMIsFeatureEnabled(name.as_ptr()) == 1 }
    } else {
        false
    }
}

/// Enables a feature for your plugin. This will change the running behavior of X-Plane
/// and plugin in some way, depending on the feature.
pub fn enable_feature(feature: Feature) {
    if let Ok(name) = ffi::CString::new(feature.name()) {
        const ENABLE: i32 = 1;
        unsafe { xplm_sys::XPLMEnableFeature(name.as_ptr(), ENABLE) };
    }
}

/// Disables a feature for plugin. This will change the running behavior of X-Plane
/// and plugin in some way, depending on the feature.
pub fn disable_feature(feature: Feature) {
    if let Ok(name) = ffi::CString::new(feature.name()) {
        const DISABLE: i32 = 0;
        unsafe { xplm_sys::XPLMEnableFeature(name.as_ptr(), DISABLE) };
    }
}
