#[derive(Debug, Default)]
pub struct DisconnectEvent;

use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
};

impl OnEvent for DisconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.pipe.disconnect(ctx.client_id)?;

        Ok(())
    }
}
