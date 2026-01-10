#![allow(unused)]
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;

use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::server_event;

pub static LOGGER: OnceLock<Logger> = OnceLock::new();

pub struct Logger {
    tx: Sender<Message>,
    level: AtomicU8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Off = 5,
}

impl Logger {
    pub fn new(tx: Sender<Message>, level: LogLevel) -> Logger {
        Logger {
            tx,
            level: AtomicU8::new(level as u8),
        }
    }

    pub fn set_level(&self, level: LogLevel) {
        self.level.store(level as u8, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn would_log(&self, level: LogLevel) -> bool {
        let current = self.level.load(Ordering::Relaxed);
        (level as u8) >= current && level != LogLevel::Off
    }

    #[inline(always)]
    pub fn log(
        &self,
        level: LogLevel,
        message: impl Into<String>,
        client_id: u32,
    ) {
        if self.would_log(level) {
            self.tx
                .send(server_event!(
                    client_id,
                    Log,
                    LogEvent::new(message.into(), level)
                ))
                .ok();
        }
    }

    #[inline(always)]
    pub fn log_cb(
        &self,
        level: LogLevel,
        client_id: u32,
        cb: impl FnOnce() -> String,
    ) {
        if self.would_log(level) {
            self.tx
                .send(server_event!(client_id, Log, LogEvent::new(cb(), level)))
                .ok();
        }
    }

    #[inline(always)]
    pub fn log_raw(
        &self,
        level: LogLevel,
        message: impl Into<String>,
        client_id: u32,
    ) {
        self.tx
            .send(server_event!(
                client_id,
                Log,
                LogEvent::new(message.into(), level)
            ))
            .ok();
    }

    #[inline(always)]
    pub fn log_raw_cb(
        &self,
        level: LogLevel,
        client_id: u32,
        cb: impl FnOnce() -> String,
    ) {
        self.tx
            .send(server_event!(client_id, Log, LogEvent::new(cb(), level)))
            .ok();
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $msg:expr, $client_id:expr) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            logger.log($level, $msg, $client_id);
        }
    }};
    ($level:expr, $msg:expr) => {
        $crate::log!($level, $msg, 0)
    };
}

#[macro_export]
macro_rules! log_raw {
    ($level:expr, $msg:expr, $client_id:expr) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            logger.log_raw($level, $msg, $client_id);
        }
    }};
    ($level:expr, $msg:expr) => {
        $crate::log_raw!($level, $msg, 0)
    };
}

#[macro_export]
macro_rules! log_cb {
    ($level:expr, $cb:expr, $client_id:expr) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            logger.log_cb($level, $client_id, $cb);
        }
    }};
    ($level:expr, $cb:expr) => {
        $crate::log_cb!($level, $cb, 0)
    };
}

#[macro_export]
macro_rules! log_raw_cb {
    ($level:expr, $cb:expr, $client_id:expr) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            logger.log_raw_cb($level, $client_id, $cb);
        }
    }};
    ($level:expr, $cb:expr) => {
        $crate::log_raw_cb!($level, $cb, 0)
    };
}

#[macro_export]
macro_rules! trace {
    // Pattern: trace!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Trace, $client_id, $fmt, $($args),*)
    };
    // Pattern: trace!(client_id, "msg")
    ($client_id:expr, $msg:literal) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Trace, $client_id, $msg)
    };
    // Pattern: trace!("format", args...)
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Trace, 0, $fmt, $($args),*)
    };
    // Pattern: trace!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Trace, $msg, 0)
    };
}

#[macro_export]
macro_rules! debug {
    // Pattern: debug!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Debug, $client_id, $fmt, $($args),*)
    };
    // Pattern: debug!(client_id, "msg")
    ($client_id:expr, $msg:literal) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Debug, $client_id, $msg)
    };
    // Pattern: debug!("format", args...)
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Debug, 0, $fmt, $($args),*)
    };
    // Pattern: debug!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Debug, $msg, 0)
    };
}

#[macro_export]
macro_rules! info {
    // Pattern: info!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Info, $client_id, $fmt, $($args),*)
    };
    // Pattern: info!(client_id, "msg")
    ($client_id:expr, $msg:literal) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Info, $client_id, $msg)
    };
    // Pattern: info!("format", args...)
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Info, 0, $fmt, $($args),*)
    };
    // Pattern: info!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Info, $msg, 0)
    };
}

#[macro_export]
macro_rules! warn {
    // Pattern: warn!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Warn, $client_id, $fmt, $($args),*)
    };
    // Pattern: warn!(client_id, "msg")
    ($client_id:expr, $msg:literal) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Warn, $client_id, $msg)
    };
    // Pattern: warn!("format", args...)
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Warn, 0, $fmt, $($args),*)
    };
    // Pattern: warn!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Warn, $msg, 0)
    };
}

#[macro_export]
macro_rules! error {
    // Pattern: error!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Error, $client_id, $fmt, $($args),*)
    };
    // Pattern: error!(client_id, "msg")
    ($client_id:expr, $msg:literal) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Error, $client_id, $msg)
    };
    // Pattern: error!("format", args...)
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::__log_fmt!($crate::util::logger::LogLevel::Error, 0, $fmt, $($args),*)
    };
    // Pattern: error!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Error, $msg, 0)
    };
}

#[macro_export]
macro_rules! __log_fmt {
    ($level:expr, $client_id:expr, $fmt:literal, $($args:expr),*) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            if logger.would_log($level) {
                logger.log_raw($level, format!($fmt, $($args),*), $client_id);
            }
        }
    }};
}

#[macro_export]
macro_rules! __log_with_client_id {
    ($level:expr, $client_id:expr, $msg:expr) => {{
        if let Some(logger) = $crate::util::logger::LOGGER.get() {
            logger.log($level, $msg, $client_id);
        }
    }};
}

impl From<u8> for LogLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => LogLevel::Off,
        }
    }
}

impl TryFrom<u64> for LogLevel {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, ()> {
        match value {
            0 => Ok(LogLevel::Trace),
            1 => Ok(LogLevel::Debug),
            2 => Ok(LogLevel::Info),
            3 => Ok(LogLevel::Warn),
            4 => Ok(LogLevel::Error),
            5 => Ok(LogLevel::Off),
            _ => Err(()),
        }
    }
}
