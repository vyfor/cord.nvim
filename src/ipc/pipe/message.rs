use std::error::Error;

use crate::json::deserialize::{Deserializable, Json};

use super::types::{Connect, Disconnect};

#[derive(Debug)]
pub enum Message {
    Client(ClientMessage),
    Server(ServerMessage),
}

#[derive(Debug)]
pub enum ClientMessage {
    Connect(Connect),
    Disconnect(Disconnect),
}

impl ClientMessage {
    pub fn deserialize(json: &str) -> Result<Self, String> {
        let map = Json::deserialize(json)?;

        let ty = map
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'type' field")?;

        let data = map
            .get("data")
            .and_then(|v| v.as_map())
            .ok_or("Missing or invalid 'data' field")?;

        Ok(match ty {
            "connect" => ClientMessage::Connect(Connect::deserialize(data)?),
            "disconnect" => ClientMessage::Disconnect(Disconnect),
            _ => return Err(format!("Unknown message type: {}", ty)),
        })
    }
}

#[derive(Debug)]
pub enum ServerMessage {
    ClientDisconnected(u32),
    Error(Box<dyn Error + Send + Sync>),
}
