use crate::messages::events::event::{EventContext, OnEvent};
use crate::trace;

#[derive(Debug, Default)]
pub struct ConnectEvent;

impl OnEvent for ConnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        trace!(ctx.client_id, "Processing connect event (no-op)");
        Ok(())
    }
}
