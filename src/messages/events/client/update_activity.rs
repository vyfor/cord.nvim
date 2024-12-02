use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::Activity;
use crate::presence::packet::Packet;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    activity: Activity,
}

impl UpdateActivityEvent {
    pub fn new(activity: Activity) -> Self {
        Self { activity }
    }
}

impl OnEvent for UpdateActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if !ctx.cord.rich_client.is_ready.load(Ordering::SeqCst) {
            return Ok(());
        }

        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            ctx.cord
                .rich_client
                .update(&Packet::new(ctx.cord.rich_client.pid, Some(&self.activity)))?;
            session.set_last_activity(self.activity);
            session.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap() // todo: reset all timestamps if this fails
                .as_nanos();
        }

        Ok(())
    }
}
