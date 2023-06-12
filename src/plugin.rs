pub trait XPlugin: Sized {
    type Error: std::error::Error;

    fn start() -> Result<Self, Self::Error>;
    fn stop(&mut self);
    fn enable(&mut self) -> Result<(), Self::Error>;
    fn disable(&mut self);
}

#[macro_export]
macro_rules! register {
    ($plugin_type: ty) => {
        use xplm::plugin::XPlugin;

        static mut PLUGIN_INSTANCE: std::sync::OnceLock<$plugin_type> = std::sync::OnceLock::new();
        const XP_RESULT_OK: ::std::os::raw::c_int = 1;
        const XP_RESULT_ERR: ::std::os::raw::c_int = 0;

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStart(
            name: *mut ::std::os::raw::c_char,
            signature: *mut ::std::os::raw::c_char,
            description: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            // Replacee with get_or_try_init after stabilization https://github.com/rust-lang/rust/issues/109737
            if PLUGIN_INSTANCE.get().is_none() {
                match <$plugin_type>::start() {
                    Ok(instance) => PLUGIN_INSTANCE
                        .set(instance)
                        .map_or(XP_RESULT_ERR, |_| XP_RESULT_OK),
                    Err(_err) => XP_RESULT_ERR,
                }
            } else {
                XP_RESULT_OK
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStop() {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.stop();
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginEnable() -> ::std::os::raw::c_int {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.enable().map_or(XP_RESULT_ERR, |_| XP_RESULT_OK)
            } else {
                XP_RESULT_ERR
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginDisable() {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.disable();
            }
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginReceiveMessage(
            from: ::std::os::raw::c_int,
            message: ::std::os::raw::c_int,
            param: *mut ::std::os::raw::c_void,
        ) {
        }
    };
}
