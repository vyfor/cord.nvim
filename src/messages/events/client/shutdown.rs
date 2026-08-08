use crate::debug;
use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ShutdownEvent;

impl OnEvent for ShutdownEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        debug!(ctx.client_id, "Processing shutdown event");
        ctx.cord.shutdown();

        Ok(())
    }
}
