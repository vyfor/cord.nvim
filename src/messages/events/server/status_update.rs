use std::collections::HashMap;
use std::fmt::Display;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::protocol::msgpack::MsgPack;
use crate::protocol::msgpack::serialize::Serialize;
use crate::protocol::msgpack::value::ValueRef;
use crate::trace;

#[derive(Debug)]
pub struct StatusUpdateEvent {
    pub status: Status,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// The client is disconnected from Discord.
    Disconnected,
    /// The client is connecting to Discord.
    Connecting,
    /// The client is connected to Discord.
    Connected,
    /// The client has handshaken with Discord.
    Ready,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "disconnected"),
            Self::Connecting => write!(f, "connecting"),
            Self::Connected => write!(f, "connected"),
            Self::Ready => write!(f, "ready"),
        }
    }
}

impl StatusUpdateEvent {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn disconnected() -> Self {
        Self::new(Status::Disconnected)
    }

    pub fn connecting() -> Self {
        Self::new(Status::Connecting)
    }

    pub fn connected() -> Self {
        Self::new(Status::Connected)
    }

    pub fn ready() -> Self {
        Self::new(Status::Ready)
    }
}

impl OnEvent for StatusUpdateEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        trace!(ctx.client_id, "Broadcasting status update: {}", self.status);
        ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;

        Ok(())
    }
}

impl Serialize for StatusUpdateEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        let mut data = HashMap::new();
        data.insert("status", ValueRef::String(self.status.to_string()));

        f("type", ValueRef::Str("status_update"), state)?;
        f("data", ValueRef::Map(data), state)?;

        Ok(())
    }
}
