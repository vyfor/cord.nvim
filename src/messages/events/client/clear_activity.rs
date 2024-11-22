#[derive(Debug, Default)]
pub struct ClearActivityEvent;

use crate::messages::events::event::{EventContext, OnEvent};

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.rich_client.clear()?;

        Ok(())
    }
}
