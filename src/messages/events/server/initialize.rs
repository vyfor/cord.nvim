use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::OnEvent;
use crate::protocol::msgpack::{Serialize, ValueRef};

#[derive(Debug, Default)]
pub struct InitializeEvent;

impl OnEvent for InitializeEvent {
    fn on_event(
        self,
        ctx: &mut crate::messages::events::event::EventContext,
    ) -> crate::Result<()> {
        ctx.cord
            .pipe
            .broadcast(&crate::protocol::msgpack::MsgPack::serialize(&self)?)?;

        Ok(())
    }
}

impl Serialize for InitializeEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        f("type", ValueRef::Str("initialize"), state)?;
        f(
            "data",
            ValueRef::String(std::process::id().to_string()),
            state,
        )?;

        Ok(())
    }
}
