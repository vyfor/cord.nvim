use std::sync::atomic::Ordering;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::EventContext;
use crate::msgpack::MsgPack;
use crate::{
    messages::events::event::OnEvent,
    msgpack::{serialize::Serialize, value::ValueRef},
};

#[derive(Debug, Default)]
pub struct ReadyEvent;

impl OnEvent for ReadyEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if !ctx.cord.rich_client.is_ready.swap(true, Ordering::SeqCst) {
            ctx.cord.pipe.broadcast(&MsgPack::serialize(&self)?)?;
        }

        Ok(())
    }
}

impl Serialize for ReadyEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::msgpack::SerializeFn<'a>,
        state: &mut crate::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::String("ready"), state)?;

        Ok(())
    }
}
