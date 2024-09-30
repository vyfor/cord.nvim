use std::ffi::CString;

use crate::CALLBACKS;

pub static mut LOG_LEVEL: u8 = 5;

pub fn init(level: u8) {
    unsafe {
        LOG_LEVEL = level;
    }
}

pub fn log(level: LogLevel, message: &str) {
    unsafe {
        if let Some(callbacks) = CALLBACKS.as_ref() {
            let level = level as i32;
            if level >= LOG_LEVEL as i32 {
                let error_message =
                    CString::new(format!("[cord.nvim] {message}")).unwrap();
                (callbacks.log_callback)(error_message.as_ptr(), level);
            }
        }
    }
}

#[allow(dead_code)]
pub fn trace(message: &str) {
    log(LogLevel::Trace, message)
}

pub fn debug(message: &str) {
    log(LogLevel::Debug, message)
}

pub fn info(message: &str) {
    log(LogLevel::Info, message)
}

pub fn warn(message: &str) {
    log(LogLevel::Warn, message)
}

pub fn error(message: &str) {
    log(LogLevel::Error, message)
}

#[repr(i32)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}
