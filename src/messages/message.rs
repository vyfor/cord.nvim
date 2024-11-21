use std::error::Error;

use crate::{
    json::deserialize::{Deserializable, Json},
    presence::activity::ActivityContext,
    types::Config,
};

#[derive(Debug)]
pub struct Message {
    pub client_id: u32,
    pub message: Event,
}

#[derive(Debug)]
pub enum Event {
    Client(ClientMessage),
    Local(LocalMessage),
}

#[derive(Debug)]
pub enum ClientMessage {
    Connect,
    Initialize(Config),
    UpdateActivity(ActivityContext),
    ClearActivity,
    UpdateWorkspace(String),
    ResetTimestamp,
    Disconnect,
}

macro_rules! data {
    ($map:expr) => {
        $map.get("data")
            .and_then(|v| v.as_map())
            .ok_or("Missing or invalid 'data' field")?
    };
    ($map:expr, $expr:expr) => {
        $map.get("data")
            .and_then($expr)
            .ok_or("Missing or invalid 'data' field")?
    };
}

impl Message {
    pub fn new(client_id: u32, message: Event) -> Self {
        Self { client_id, message }
    }
}

impl ClientMessage {
    // { type: string, data: any }
    pub fn deserialize(json: &str) -> Result<Self, String> {
        let map = Json::deserialize(json)?;

        let ty = map
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'type' field")?;

        Ok(match ty {
            "connect" => ClientMessage::Connect,
            "initialize" => ClientMessage::Initialize(Config::deserialize(data!(map))?),
            "update_activity" => {
                ClientMessage::UpdateActivity(ActivityContext::deserialize(data!(map))?)
            }
            "clear_activity" => ClientMessage::ClearActivity,
            "update_workspace" => ClientMessage::UpdateWorkspace(data!(map, |v| v.as_string())),
            "reset_timestamp" => ClientMessage::ResetTimestamp,
            "disconnect" => ClientMessage::Disconnect,
            _ => return Err(format!("Unknown message type: {}", ty)),
        })
    }
}

#[derive(Debug)]
pub enum LocalMessage {
    ClientDisconnected,
    Error(Box<dyn Error + Send + Sync>),
}
