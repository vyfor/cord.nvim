use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::protocol::msgpack::{MsgPack, Serialize, ValueRef};

#[derive(Debug, Default)]
pub struct ShutdownEvent;

impl OnEvent for ShutdownEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;
        ctx.cord.shutdown();

        Ok(())
    }
}

impl Serialize for ShutdownEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::String("shutdown"), state)?;

        Ok(())
    }
}
