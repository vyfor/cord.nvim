use crate::messages::events::event::{EventContext, OnEvent};
use crate::debug;

#[derive(Debug, Default)]
pub struct ShutdownEvent;

impl OnEvent for ShutdownEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        debug!(ctx.client_id, "Processing shutdown event");
        ctx.cord.shutdown();

        Ok(())
    }
}
