use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ClearActivityEvent;

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            ctx.cord.rich_client.clear()?;
            session.last_activity = None;
        }
        Ok(())
    }
}
