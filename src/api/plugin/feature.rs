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
