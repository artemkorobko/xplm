pub mod error;
pub mod feature;
pub mod id;
pub mod info;
pub mod message;

use std::{ffi, ops::Deref};

pub use self::error::PluginError;
pub use self::feature::Feature;
pub use self::id::PluginId;
pub use self::info::PluginInfo;
pub use self::message::AsMessageParam;

pub type Result<T> = std::result::Result<T, PluginError>;

/// Returns the plugin ID of the calling plug-in. Call this to get your own ID.
///
/// # Returns
/// Returns [`PluginId`] in case of success. Otherwise returns [`PluginError`].
pub fn get_my_id() -> Result<PluginId> {
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
/// Returns [`PluginId`] in case of success. Otherwise returns [`PluginError`].
pub fn get_nth_plugin(index: usize) -> Result<PluginId> {
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
/// * [`PluginError::InvalidPluginPath`] if path contains invalid characters.
pub fn find_plugin_by_path<T: Into<String>>(path: T) -> Result<PluginId> {
    let string_c = ffi::CString::new(path.into()).map_err(PluginError::InvalidPluginPath)?;
    let id = unsafe { xplm_sys::XPLMFindPluginByPath(string_c.as_ptr()) };
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
/// * [`PluginError::InvalidPluginSignature`] if path contains invalid characters.
pub fn find_plugin_by_signature<T: Into<String>>(signature: T) -> Result<PluginId> {
    let string_c =
        ffi::CString::new(signature.into()).map_err(PluginError::InvalidPluginSignature)?;
    let id = unsafe { xplm_sys::XPLMFindPluginBySignature(string_c.as_ptr()) };
    PluginId::try_from(id)
}

/// Returns information about a plug-in.
///
/// # Arguments
/// * `id` - the plugin identifier. See [`PluginId`].
///
/// # Returns
/// Returns [`PluginInfo`] in case of success. Otherwise returns [`PluginError`].
pub fn get_plugin_info(id: &PluginId) -> Result<PluginInfo> {
    let (name, file_path, signature, description) = unsafe {
        const BUF_LEN: usize = 256;
        let mut out_name = [0; BUF_LEN];
        let mut out_file_path = [0; BUF_LEN];
        let mut out_signature = [0; BUF_LEN];
        let mut out_description = [0; BUF_LEN];

        xplm_sys::XPLMGetPluginInfo(
            *id.deref(),
            out_name.as_mut_ptr(),
            out_file_path.as_mut_ptr(),
            out_signature.as_mut_ptr(),
            out_description.as_mut_ptr(),
        );

        let name = ffi::CStr::from_ptr(out_name.as_ptr())
            .to_owned()
            .into_string()
            .map_err(PluginError::InvalidInfoName)?;
        let file_path = ffi::CStr::from_ptr(out_file_path.as_ptr())
            .to_owned()
            .into_string()
            .map_err(PluginError::InvalidInfoFilePath)?;
        let signature = ffi::CStr::from_ptr(out_signature.as_ptr())
            .to_owned()
            .into_string()
            .map_err(PluginError::InvalidInfoSignature)?;
        let description = ffi::CStr::from_ptr(out_description.as_ptr())
            .to_owned()
            .into_string()
            .map_err(PluginError::InvalidInfoDescription)?;

        (name, file_path, signature, description)
    };

    Ok(PluginInfo {
        name,
        file_path,
        signature,
        description,
    })
}

/// Returns whether the specified plug-in is enabled for running.
///
/// # Arguments
/// * `id` - the plugin identifier.
///
/// # Returns
/// Return `true` in case the plugin is enabled. Otherwise returns `false`.
pub fn is_plugin_enabled(id: &PluginId) -> bool {
    unsafe { xplm_sys::XPLMIsPluginEnabled(*id.deref()) == 1 }
}

/// Enables a plug-in if it is not already enabled. Plugins may fail to enable
/// (for example, if resources cannot be acquired) by returning 0 from their XPluginEnable callback.
///
/// # Arguments
/// * `id` - the plugin identifier.
///
/// # Returns
/// Returns `true` if the plugin was enabled or successfully enables itself. Otherwise returns `false`.
pub fn enable_plugin(id: &PluginId) -> bool {
    unsafe { xplm_sys::XPLMEnablePlugin(*id.deref()) == 1 }
}

/// Disables an enabled plug-in.
pub fn disable_plugin(id: &PluginId) {
    unsafe { xplm_sys::XPLMDisablePlugin(*id.deref()) };
}

/// Reloads all plug-ins. Once this routine is called and you return from the callback
/// you were within (e.g. a menu select callback) you will receive your XPluginDisable
/// and XPluginStop callbacks and your DLL will be unloaded, then the start process happens
/// as if the sim was starting up.
pub fn reload_plugins() {
    unsafe { xplm_sys::XPLMReloadPlugins() };
}

/// Sends a message to another plug-in or X-Plane. Only enabled plug-ins with a message
/// receive function receive the message.
///
/// # Arguments
/// * `id` - the plugin identifier.
/// * `message` - the unique message identifier.
/// * `param` - the message param.
pub fn send_message_to_plugin<P: AsMessageParam>(id: &PluginId, message: i32, param: P) {
    unsafe { xplm_sys::XPLMSendMessageToPlugin(*id.deref(), message, param.as_message_param()) };
}

/// Broadcasts a message to all plug-ins. Only enabled plug-ins with a message
/// receive function receive the message.
///
/// # Arguments
/// * `message` - the unique message identifier.
/// * `param` - the message param.
pub fn send_message_to_all_plugins<P: AsMessageParam>(message: i32, param: P) {
    unsafe {
        xplm_sys::XPLMSendMessageToPlugin(
            xplm_sys::XPLM_NO_PLUGIN_ID,
            message,
            param.as_message_param(),
        )
    };
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
        unsafe { xplm_sys::XPLMEnableFeature(name.as_ptr(), 1) };
    }
}

/// Disables a feature for plugin. This will change the running behavior of X-Plane
/// and plugin in some way, depending on the feature.
pub fn disable_feature(feature: Feature) {
    if let Ok(name) = ffi::CString::new(feature.name()) {
        unsafe { xplm_sys::XPLMEnableFeature(name.as_ptr(), 0) };
    }
}
