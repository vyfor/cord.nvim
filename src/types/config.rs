#![allow(dead_code)]

use crate::{
    get_field,
    protocol::msgpack::{deserialize::Deserialize, Value},
    util::logger::LogLevel,
};

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub log_level: LogLevel,
}

impl Deserialize for PluginConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let input = input.take_map().ok_or("Invalid config")?;

        let log_level = get_field!(input, "log_level", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid log level")?;

        Ok(PluginConfig { log_level })
    }
}
