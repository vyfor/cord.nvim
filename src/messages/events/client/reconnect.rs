use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::local::ReconnectEvent;
use crate::{debug, local_event};

#[derive(Debug, Default)]
pub struct ReconnectClientEvent;

impl OnEvent for ReconnectClientEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        debug!(ctx.client_id, "Processing reconnect client event");

        let _ = ctx
            .cord
            .tx
            .send(local_event!(ctx.client_id, Reconnect, ReconnectEvent::new(true)));

        Ok(())
    }
}
