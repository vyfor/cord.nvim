use crate::messages::events::event::{EventContext, OnEvent};
use crate::server_event;

#[derive(Debug, Default)]
pub struct ShutdownEvent;

impl OnEvent for ShutdownEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.tx.send(server_event!(0, Shutdown)).ok();

        Ok(())
    }
}
