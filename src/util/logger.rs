pub static mut LOG_LEVEL: u8 = 5; // OFF

pub fn init(level: u8) {
    unsafe {
        LOG_LEVEL = level;
    }
}

#[allow(unused_imports, unused_macros)]
#[macro_use]
pub mod log {
    #[macro_export]
    macro_rules! log {
        ($level:expr, $($arg:tt)*) => {{
            let callbacks;
            let log_level;
            unsafe {
                callbacks = $crate::CALLBACKS.as_ref();
                log_level = $crate::logger::LOG_LEVEL as i32;
            }
            if let Some(callbacks) = callbacks {
                let level = $level as i32;
                if level >= log_level {
                    let error_message = std::ffi::CString::new(format!("[cord.nvim] {}", format_args!($($arg)*))).unwrap();
                    (callbacks.log_callback)(error_message.as_ptr(), level);
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            $crate::log!($crate::logger::LogLevel::Trace, $($arg)*)
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            $crate::log!($crate::logger::LogLevel::Debug, $($arg)*)
        };
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            $crate::log!($crate::logger::LogLevel::Info, $($arg)*)
        };
    }

    #[macro_export]
    macro_rules! warning {
        ($($arg:tt)*) => {
            $crate::log!($crate::logger::LogLevel::Warn, $($arg)*)
        };
    }

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            $crate::log!($crate::logger::LogLevel::Error, $($arg)*)
        };
    }
}

#[repr(i32)]
pub enum LogLevel {
    #[allow(dead_code)]
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}
