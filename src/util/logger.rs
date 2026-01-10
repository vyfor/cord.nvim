#![allow(unused)]
use std::sync::mpsc::Sender;
use std::sync::{OnceLock, RwLock};

use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::server_event;

pub static INSTANCE: OnceLock<RwLock<Logger>> = OnceLock::new();

pub struct Logger {
    tx: Sender<Message>,
    level: LogLevel,
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
        Logger { tx, level }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    #[inline(always)]
    pub fn log(
        &self,
        level: LogLevel,
        message: impl Into<String>,
        client_id: u32,
    ) {
        if level >= self.level && level != LogLevel::Off {
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
        if level >= self.level && level != LogLevel::Off {
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
        if let Some(logger) = $crate::util::logger::INSTANCE.get() {
            let logger = logger.read().unwrap();
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
        if let Some(logger) = $crate::util::logger::INSTANCE.get() {
            let logger = logger.read().unwrap();
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
        if let Some(logger) = $crate::util::logger::INSTANCE.get() {
            let logger = logger.read().unwrap();
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
        if let Some(logger) = $crate::util::logger::INSTANCE.get() {
            let logger = logger.read().unwrap();
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
    ($client_id:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Trace, $client_id, format!($fmt, $($args),*))
    };
    // Pattern: trace!("format", args...)
    ($fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Trace, 0, format!($fmt, $($args),*))
    };
    // Pattern: trace!(client_id, msg)
    ($client_id:literal, $msg:expr) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Trace, $client_id, $msg)
    };
    // Pattern: trace!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Trace, $msg, 0)
    };
}

#[macro_export]
macro_rules! debug {
    // Pattern: debug!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Debug, $client_id, format!($fmt, $($args),*))
    };
    // Pattern: debug!("format", args...)
    ($fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Debug, 0, format!($fmt, $($args),*))
    };
    // Pattern: debug!(client_id, msg)
    ($client_id:literal, $msg:expr) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Debug, $client_id, $msg)
    };
    // Pattern: debug!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Debug, $msg, 0)
    };
}

#[macro_export]
macro_rules! info {
    // Pattern: info!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Info, $client_id, format!($fmt, $($args),*))
    };
    // Pattern: info!("format", args...)
    ($fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Info, 0, format!($fmt, $($args),*))
    };
    // Pattern: info!(client_id, msg)
    ($client_id:literal, $msg:expr) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Info, $client_id, $msg)
    };
    // Pattern: info!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Info, $msg, 0)
    };
}

#[macro_export]
macro_rules! warn {
    // Pattern: warn!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Warn, $client_id, format!($fmt, $($args),*))
    };
    // Pattern: warn!("format", args...)
    ($fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Warn, 0, format!($fmt, $($args),*))
    };
    // Pattern: warn!(client_id, msg)
    ($client_id:literal, $msg:expr) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Warn, $client_id, $msg)
    };
    // Pattern: warn!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Warn, $msg, 0)
    };
}

#[macro_export]
macro_rules! error {
    // Pattern: error!(client_id, "format", args...)
    ($client_id:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Error, $client_id, format!($fmt, $($args),*))
    };
    // Pattern: error!("format", args...)
    ($fmt:literal, $($args:expr),* $(,)?) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Error, 0, format!($fmt, $($args),*))
    };
    // Pattern: error!(client_id, msg)
    ($client_id:literal, $msg:expr) => {
        $crate::__log_with_client_id!($crate::util::logger::LogLevel::Error, $client_id, $msg)
    };
    // Pattern: error!(msg)
    ($msg:expr) => {
        $crate::log!($crate::util::logger::LogLevel::Error, $msg, 0)
    };
}

#[macro_export]
macro_rules! __log_with_client_id {
    ($level:expr, $client_id:expr, $msg:expr) => {{
        if let Some(logger) = $crate::util::logger::INSTANCE.get() {
            let logger = logger.read().unwrap();
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
