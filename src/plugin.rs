pub trait XPlugin: Sized {
    type Error: std::error::Error;

    fn start() -> Result<Self, Self::Error>;
    fn stop(&mut self);
    fn enable(&mut self) -> Result<(), Self::Error>;
    fn disable(&mut self);
}

#[macro_export]
macro_rules! register_plugin {
    (
        instance = $plugin_type: ty,
        name = $name: literal,
        signature = $signature: literal,
        description = $description: literal,
        log_prefix = $log_prefix: literal,
    ) => {
        use xplm::plugin::XPlugin;

        static mut PLUGIN_INSTANCE: std::sync::OnceLock<$plugin_type> = std::sync::OnceLock::new();
        const XP_RESULT_OK: ::std::os::raw::c_int = 1;
        const XP_RESULT_ERR: ::std::os::raw::c_int = 0;

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginStart(
            name: *mut ::std::os::raw::c_char,
            signature: *mut ::std::os::raw::c_char,
            description: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            pub unsafe fn copy_to_c_buffer(src: &str, dest: *mut ::std::os::raw::c_char) {
                let src_len = std::cmp::min(src.len(), 255);
                let src_c = std::ffi::CString::new(&src[..src_len])
                    .unwrap_or_else(|_| std::ffi::CString::new("<invalid>").unwrap());
                let src_c_length = src_c.to_bytes_with_nul().len();
                std::ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length);
            }

            xplm::log::LOG_PREFIX.set($log_prefix);

            // Replacee with get_or_try_init after stabilization https://github.com/rust-lang/rust/issues/109737
            if PLUGIN_INSTANCE.get().is_none() {
                match <$plugin_type>::start() {
                    Ok(instance) => {
                        copy_to_c_buffer($name, name);
                        copy_to_c_buffer($signature, signature);
                        copy_to_c_buffer($description, description);

                        PLUGIN_INSTANCE
                            .set(instance)
                            .map_or(XP_RESULT_ERR, |_| XP_RESULT_OK)
                    }
                    Err(_err) => XP_RESULT_ERR,
                }
            } else {
                XP_RESULT_OK
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginStop() {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.stop();
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginEnable() -> ::std::os::raw::c_int {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.enable().map_or(XP_RESULT_ERR, |_| XP_RESULT_OK)
            } else {
                XP_RESULT_ERR
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginDisable() {
            if let Some(instance) = PLUGIN_INSTANCE.get_mut() {
                instance.disable();
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn XPluginReceiveMessage(
            _from: ::std::os::raw::c_int,
            _message: ::std::os::raw::c_int,
            _param: *mut ::std::os::raw::c_void,
        ) {
        }
    };
}
