use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
};

#[derive(Debug, Default)]
pub struct ClientDisconnectedEvent;

impl OnEvent for ClientDisconnectedEvent {
    fn on_event(self, ctx: &EventContext) -> crate::Result<()> {
        ctx.pipe.disconnect(ctx.client_id)?;

        Ok(())
    }
}
