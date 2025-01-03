use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::protocol::msgpack::serialize::Serialize;
use crate::protocol::msgpack::value::ValueRef;
use crate::protocol::msgpack::MsgPack;

#[derive(Debug, Default)]
pub struct DisconnectEvent;

impl OnEvent for DisconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;

        Ok(())
    }
}

impl Serialize for DisconnectEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::Str("disconnect"), state)?;
        f("data", ValueRef::Nil, state)?;

        Ok(())
    }
}
