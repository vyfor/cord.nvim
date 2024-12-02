use crate::{
    messages::events::event::{EventContext, OnEvent},
    presence::activity::Activity,
    protocol::msgpack::{Deserialize, MsgPack},
    types::config::PluginConfig,
};

pub mod clear_activity;
pub mod connect;
pub mod disconnect;
pub mod initialize;
pub mod update_activity;

pub use clear_activity::ClearActivityEvent;
pub use connect::ConnectEvent;
pub use disconnect::DisconnectEvent;
pub use initialize::InitializeEvent;
pub use update_activity::UpdateActivityEvent;

#[derive(Debug)]
pub enum ClientEvent {
    Connect(ConnectEvent),
    Initialize(InitializeEvent),
    UpdateActivity(UpdateActivityEvent),
    ClearActivity(ClearActivityEvent),
    Disconnect(DisconnectEvent),
}

/// Extracts the 'data' field from a map and returns an error if it is missing or invalid.
///
/// This macro is useful for extracting and validating the 'data' field from a map-like structure.
macro_rules! data {
    ($map:expr) => {
        $map.remove("data")
            .ok_or("Missing or invalid 'data' field")?
    };
    ($map:expr, $expr:expr) => {
        $map.remove("data")
            .and_then($expr)
            .ok_or("Missing or invalid 'data' field")?
    };
}

impl ClientEvent {
    // { type: string, data: any }
    pub fn deserialize(json: &[u8]) -> crate::Result<Self> {
        let mut map = MsgPack::deserialize(json)?
            .take_map()
            .ok_or("Invalid message")?;

        let ty = map
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'type' field")?;

        Ok(match ty {
            "connect" => Self::Connect(ConnectEvent),
            "initialize" => {
                Self::Initialize(InitializeEvent::new(PluginConfig::deserialize(data!(map))?))
            }
            "update_activity" => {
                Self::UpdateActivity(UpdateActivityEvent::new(Activity::deserialize(data!(map))?))
            }
            "clear_activity" => Self::ClearActivity(ClearActivityEvent),
            "disconnect" => Self::Disconnect(DisconnectEvent),
            _ => return Err(format!("Unknown message type: {}", ty).into()),
        })
    }
}

impl OnEvent for ClientEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Initialize(e) => e.on_event(ctx),
            Self::Connect(e) => e.on_event(ctx),
            Self::Disconnect(e) => e.on_event(ctx),
            Self::UpdateActivity(e) => e.on_event(ctx),
            Self::ClearActivity(e) => e.on_event(ctx),
        }
    }
}
