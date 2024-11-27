use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct SetTimestampEvent {
    pub timestamp: Option<u64>,
}

impl OnEvent for SetTimestampEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            session.timestamp = self.timestamp;
        }

        Ok(())
    }
}

impl SetTimestampEvent {
    pub fn new(timestamp: Option<u64>) -> Self {
        Self { timestamp }
    }
}
