use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
};

#[derive(Debug, Default)]
pub struct ClientDisconnectedEvent;

impl OnEvent for ClientDisconnectedEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.pipe.disconnect(ctx.client_id)?;

        Ok(())
    }
}