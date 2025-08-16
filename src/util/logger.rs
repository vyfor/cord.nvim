#![allow(unused)]
use std::sync::mpsc::Sender;

use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::server_event;

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

    pub fn trace(&self, message: impl Into<String>, client_id: u32) {
        self.log(LogLevel::Trace, message.into(), client_id);
    }

    pub fn debug(&self, message: impl Into<String>, client_id: u32) {
        self.log(LogLevel::Debug, message.into(), client_id);
    }

    pub fn info(&self, message: impl Into<String>, client_id: u32) {
        self.log(LogLevel::Info, message.into(), client_id);
    }

    pub fn warn(&self, message: impl Into<String>, client_id: u32) {
        self.log(LogLevel::Warn, message.into(), client_id);
    }

    pub fn error(&self, message: impl Into<String>, client_id: u32) {
        self.log(LogLevel::Error, message.into(), client_id);
    }

    #[inline]
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
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
