pub enum Level {
    Info,
    Warn,
    Error,
}

pub static LOG_PREFIX: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

#[macro_export]
macro_rules! log {
    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => {{
        let args = format_args!($($arg)*);
        let level = match $lvl {
            $crate::log::Level::Info => "INFO",
            $crate::log::Level::Warn => "WARNING",
            $crate::log::Level::Error => "ERROR",
        };

        let message = if let Some(prefix) = $crate::log::LOG_PREFIX.get() {
            format!("{} {}: {}\n", prefix, level, args)
        } else {
            format!("{}: {}\n", level, args)
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
