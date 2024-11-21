use crate::{
    json::deserialize::{Deserializable, Json},
    presence::activity::ActivityContext,
    types::Config,
};

pub mod clear_activity;
pub mod connect;
pub mod disconnect;
pub mod initialize;
pub mod reset_timestamp;
pub mod update_activity;
pub mod update_workspace;

pub use clear_activity::ClearActivityEvent;
pub use connect::ConnectEvent;
pub use disconnect::DisconnectEvent;
pub use initialize::InitializeEvent;
pub use reset_timestamp::ResetTimestampEvent;
pub use update_activity::UpdateActivityEvent;
pub use update_workspace::UpdateWorkspaceEvent;

#[derive(Debug)]
pub enum ClientEvent {
    Connect(ConnectEvent),
    Initialize(InitializeEvent),
    UpdateActivity(UpdateActivityEvent),
    ClearActivity(ClearActivityEvent),
    UpdateWorkspace(UpdateWorkspaceEvent),
    ResetTimestamp(ResetTimestampEvent),
    Disconnect(DisconnectEvent),
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

impl ClientEvent {
    // { type: string, data: any }
    pub fn deserialize(json: &str) -> Result<Self, String> {
        let map = Json::deserialize(json)?;

        let ty = map
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'type' field")?;

        Ok(match ty {
            "connect" => Self::Connect(ConnectEvent),
            "initialize" => {
                Self::Initialize(InitializeEvent::new(Config::deserialize(data!(map))?))
            }
            "update_activity" => Self::UpdateActivity(UpdateActivityEvent::new(
                ActivityContext::deserialize(data!(map))?,
            )),
            "clear_activity" => Self::ClearActivity(ClearActivityEvent),
            "update_workspace" => {
                Self::UpdateWorkspace(UpdateWorkspaceEvent::new(data!(map, |v| v.as_string())))
            }
            "reset_timestamp" => Self::ResetTimestamp(ResetTimestampEvent),
            "disconnect" => Self::Disconnect(DisconnectEvent),
            _ => return Err(format!("Unknown message type: {}", ty)),
        })
    }
}
