use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
};

#[derive(Debug, Default)]
pub struct DisconnectEvent;

impl OnEvent for DisconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.session_manager.remove_session(ctx.client_id);
        ctx.cord.pipe.disconnect(ctx.client_id)?;

        Ok(())
    }
}
