#![allow(dead_code)]

use crate::protocol::msgpack::Value;
use crate::protocol::msgpack::deserialize::Deserialize;
use crate::remove_field;
use crate::util::logger::LogLevel;

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub log_level: LogLevel,
    pub timestamp: TimestampConfig,
}

#[derive(Debug, Clone)]
pub struct TimestampConfig {
    pub shared: bool,
}

impl Deserialize for TimestampConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let shared = remove_field!(input, "shared", |v| v.as_bool());

        Ok(TimestampConfig { shared })
    }
}

impl Deserialize for PluginConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let log_level = remove_field!(input, "log_level", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid log level")?;
        let timestamp = remove_field!(input, "timestamp", |v| {
            TimestampConfig::deserialize(v).ok()
        });

        Ok(PluginConfig {
            log_level,
            timestamp,
        })
    }
}
