#![allow(dead_code)]

use crate::protocol::msgpack::Value;
use crate::protocol::msgpack::deserialize::Deserialize;
use crate::util::logger::LogLevel;
use crate::{remove_field, remove_field_or_none};

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub log_level: LogLevel,
    pub timestamp: TimestampConfig,
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct AdvancedConfig {
    pub discord: AdvancedDiscordConfig,
}

impl Deserialize for AdvancedConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let discord = remove_field!(input, "discord", |v| {
            AdvancedDiscordConfig::deserialize(v).ok()
        });

        Ok(AdvancedConfig { discord })
    }
}

#[derive(Debug, Clone, Default)]
pub struct AdvancedDiscordConfig {
    pub pipe_paths: Vec<String>,
}

impl Deserialize for AdvancedDiscordConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let pipe_paths = remove_field_or_none!(input, "pipe_paths", |v| {
            v.take_array().and_then(|arr| {
                arr.into_iter()
                    .map(|v| {
                        v.take_string()
                            .ok_or("Invalid discord pipe path: not a string")
                    })
                    .collect::<Result<Vec<String>, _>>()
                    .ok()
            })
        })
        .unwrap_or_default();

        Ok(AdvancedDiscordConfig { pipe_paths })
    }
}

impl Deserialize for PluginConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let log_level = remove_field!(input, "log_level", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid log level")?;
        let timestamp = remove_field_or_none!(input, "timestamp", |v| {
            TimestampConfig::deserialize(v).ok()
        })
        .unwrap_or_default();
        let advanced = remove_field_or_none!(input, "advanced", |v| {
            AdvancedConfig::deserialize(v).ok()
        })
        .unwrap_or_default();

        Ok(PluginConfig {
            log_level,
            timestamp,
            advanced,
        })
    }
}
