use crate::{
    debug,
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
    protocol::msgpack::{MsgPack, Serialize, ValueRef},
};

#[derive(Debug, Default)]
pub struct RestartEvent;

impl OnEvent for RestartEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        debug!(ctx.client_id, "Processing restart event, broadcasting to clients");
        ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;
        ctx.cord.shutdown();

        Ok(())
    }
}

impl Serialize for RestartEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::Str("restart"), state)?;
        f("data", ValueRef::Nil, state)?;

        Ok(())
    }
}
