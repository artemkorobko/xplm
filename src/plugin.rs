use crate::api::plugin::Message;

/// The trait that all plugins should implement
pub trait XPlugin: Sized {
    /// The error type that a plugin may encounter when starting up or enabling
    type Error;

    /// Called when X-Plane loads this plugin
    ///
    /// On success, returns a plugin instance
    fn start() -> Result<Self, Self::Error>;

    /// Called when X-Plane unloads this plugin
    fn stop(self) {}

    /// Called when the plugin is enabled
    ///
    /// If this function returns an Err, the plugin will remain disabled.
    ///
    /// The default implementation returns Ok(()).
    fn enable(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the plugin is disabled
    ///
    /// The default implementation does nothing.
    fn disable(&mut self) {}

    /// Called when the plugin receives a message
    ///
    /// The default implementation does nothing.
    fn receive_message(&mut self, _: Message) {}
}

/// By convention, plugin-defined notifications should have the high bit set
/// (e.g. be greater or equal to unsigned 0x8000000) while commands should have this bit be cleared.
pub const MIN_MESSAGE_ID: i32 = 0x8000000;

#[macro_export]
macro_rules! register_plugin {
    (
        instance = $plugin_type: ty,
        name = $name: literal,
        signature = $signature: literal,
        description = $description: literal,
    ) => {
        use xplm::sys;

        static mut PLUGIN_INSTANCE: std::sync::OnceLock<$plugin_type> = std::sync::OnceLock::new();
        const XP_RESULT_OK: ::std::os::raw::c_int = 1;
        const XP_RESULT_ERR: ::std::os::raw::c_int = 0;

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginStart(
            name: *mut ::std::os::raw::c_char,
            signature: *mut ::std::os::raw::c_char,
            description: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            pub fn copy_to_c_buffer(src: &str, dest: *mut ::std::os::raw::c_char) {
                let src_len = std::cmp::min(src.len(), 255);
                let src_c = std::ffi::CString::new(&src[..src_len])
                    .unwrap_or_else(|_| std::ffi::CString::new("<invalid>").unwrap());
                let src_c_length = src_c.to_bytes_with_nul().len();
                unsafe { std::ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length) };
            }

            // Replace with get_or_try_init after stabilization https://github.com/rust-lang/rust/issues/109737
            if PLUGIN_INSTANCE.get().is_some() {
                return XP_RESULT_OK;
            }

            match <$plugin_type>::start() {
                Ok(instance) => {
                    copy_to_c_buffer($name, name);
                    copy_to_c_buffer($signature, signature);
                    copy_to_c_buffer($description, description);

                    PLUGIN_INSTANCE
                        .set(instance)
                        .map_or(XP_RESULT_ERR, |_| XP_RESULT_OK)
                }
                Err(err) => {
                    xplm::error!("{}", err);
                    XP_RESULT_ERR
                }
            }
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginStop() {
            if let Some(mut instance) = PLUGIN_INSTANCE.take() {
                instance.stop();
            }
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginEnable() -> ::std::os::raw::c_int {
            let Some(instance) = PLUGIN_INSTANCE.get_mut() else {
                return XP_RESULT_ERR;
            };

            if let Err(err) = instance.enable() {
                xplm::error!("{}", err);
                XP_RESULT_ERR
            } else {
                XP_RESULT_OK
            }
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginDisable() {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.disable();
            }
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginReceiveMessage(
            from: ::std::os::raw::c_int,
            message: ::std::os::raw::c_int,
            param: *mut ::std::os::raw::c_void,
        ) {
            let Some(instance) = PLUGIN_INSTANCE.get_mut() else {
                return;
            };

            let msg = match message as u32 {
                sys::XPLM_MSG_PLANE_CRASHED => Message::PlaneCrashed,
                sys::XPLM_MSG_PLANE_LOADED => Message::PlaneLoaded(param as usize),
                sys::XPLM_MSG_AIRPORT_LOADED => Message::AirportLoaded,
                sys::XPLM_MSG_SCENERY_LOADED => Message::SceneryLoaded,
                sys::XPLM_MSG_AIRPLANE_COUNT_CHANGED => Message::AirplaneCountChanged,
                sys::XPLM_MSG_PLANE_UNLOADED => Message::PlaneUnloaded(param as usize),
                sys::XPLM_MSG_WILL_WRITE_PREFS => Message::WillWritePrefs,
                sys::XPLM_MSG_LIVERY_LOADED => Message::LiveryLoaded(param as usize),
                sys::XPLM_MSG_ENTERED_VR => Message::EnteredVr,
                sys::XPLM_MSG_EXITING_VR => Message::ExitingVr,
                sys::XPLM_MSG_RELEASE_PLANES => Message::ReleasePlanes,
                sys::XPLM_MSG_FMOD_BANK_LOADED => Message::FmodBankLoaded(param as usize),
                sys::XPLM_MSG_FMOD_BANK_UNLOADING => Message::FmodBankUnloading(param as usize),
                sys::XPLM_MSG_DATAREFS_ADDED => Message::DataRefsAdded(param as usize),
                _ => Message::Custom(from as usize, message as usize, param),
            };

            instance.receive_message(msg);
        }
    };
}
