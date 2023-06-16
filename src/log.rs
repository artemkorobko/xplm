pub enum Level {
    Info,
    Warn,
    Error,
}

#[macro_export]
macro_rules! log {
    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => {{
        let args = format_args!($($arg)*);
        let module = module_path!();
        let message = match $lvl {
            $crate::log::Level::Info => format!("[INFO {}] {}\n", module, args),
            $crate::log::Level::Warn => format!("[WARN {}] {}\n", module, args),
            $crate::log::Level::Error => format!("[ERROR {}:{}] {}\n", module, line!(), args),
        };
        $crate::api::utilities::debug_string(message);
    }};
}

#[macro_export]
macro_rules! error {
    // error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Error, $($arg)+))
}

#[macro_export]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Warn, $($arg)+))
}

#[macro_export]
macro_rules! info {
    // info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Info, $($arg)+))
}
