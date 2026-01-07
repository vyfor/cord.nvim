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
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncMode {
    Periodic,
    Defer,
}

impl Default for SyncMode {
    fn default() -> Self {
        Self::Periodic
    }
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub enabled: bool,
    pub mode: SyncMode,
    pub interval: u64,
    pub reset_on_update: bool,
    pub pad: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: SyncMode::default(),
            interval: 12000,
            reset_on_update: true,
            pad: true,
        }
    }
}

impl Deserialize for SyncConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let enabled = remove_field!(input, "enabled", |v| v.as_bool());
        let mode = remove_field!(input, "mode", |v| {
            v.as_str().and_then(|s| match s {
                "periodic" => Some(SyncMode::Periodic),
                "defer" => Some(SyncMode::Defer),
                _ => None,
            })
        });
        let interval = remove_field!(input, "interval", |v| v.as_uinteger());
        let pad = remove_field!(input, "pad", |v| v.as_bool());
        let reset_on_update =
            remove_field!(input, "reset_on_update", |v| v.as_bool());

        Ok(SyncConfig {
            enabled,
            mode,
            interval,
            reset_on_update,
            pad,
        })
    }
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

        let sync = remove_field_or_none!(input, "sync", |v| {
            SyncConfig::deserialize(v).ok()
        })
        .unwrap_or_default();

        Ok(AdvancedDiscordConfig { pipe_paths, sync })
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
