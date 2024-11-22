use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ResetTimestampEvent;

impl OnEvent for ResetTimestampEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(config) = &mut ctx.cord.config {
            config.timestamp = None;
        }

        Ok(())
    }
}
