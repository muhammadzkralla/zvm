// Debug logging macro - controlled by feature flag
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-logging")]
        println!($($arg)*);
    };
}
