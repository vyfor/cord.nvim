#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
}

use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
    protocol::msgpack::{MsgPack, Serialize, ValueRef},
};

impl OnEvent for LogEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let message = MsgPack::serialize(&self)?;

        match ctx.client_id {
            0 => ctx.cord.pipe.broadcast(&message)?,
            _ => ctx.cord.pipe.write_to(ctx.client_id, &message)?,
        };

        Ok(())
    }
}

impl LogEvent {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Serialize for LogEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::String("log"), state)?;
        f("data", ValueRef::String(&self.message), state)?;

        Ok(())
    }
}
