use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ResetTimestampEvent;

impl OnEvent for ResetTimestampEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            session.timestamp = None;
        }

        Ok(())
    }
}
