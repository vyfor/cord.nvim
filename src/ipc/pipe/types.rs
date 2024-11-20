use std::collections::HashMap;

use crate::{
    json::deserialize::{Deserializable, Value},
    types::Config,
};

#[derive(Debug)]
pub struct Connect {
    pub config: Config,
}

impl Deserializable for Connect {
    fn deserialize(map: &HashMap<String, Value>) -> Result<Self, String> {
        let config = Config::deserialize(map)?;

        Ok(Connect { config })
    }
}

#[derive(Debug)]
pub struct Disconnect;

#[derive(Debug)]
pub struct ClientDisconnected(u32);

impl Deserializable for ClientDisconnected {
    fn deserialize(map: &HashMap<String, Value>) -> Result<Self, String> {
        let code = map
            .get("code")
            .and_then(|v| v.as_number())
            .ok_or("Missing 'code' field")? as u32;

        Ok(ClientDisconnected(code))
    }
}
