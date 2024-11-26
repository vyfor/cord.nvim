use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ClearActivityEvent;

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(_session) = ctx.cord.session_manager.get_session(ctx.client_id) {
            ctx.cord.rich_client.clear()?;
        }
        Ok(())
    }
}
