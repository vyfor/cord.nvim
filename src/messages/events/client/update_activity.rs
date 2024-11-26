use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::ActivityContext;
use crate::presence::types::Packet;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    context: ActivityContext,
}

impl UpdateActivityEvent {
    pub fn new(context: ActivityContext) -> Self {
        Self { context }
    }
}

impl OnEvent for UpdateActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if !ctx.cord.rich_client.is_ready.load(Ordering::SeqCst) {
            return Ok(());
        }

        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            if let Some(config) = session.get_config() {
                let activity = self.context.build(config);
                if Some(&activity) != session.last_activity.as_ref() {
                    ctx.cord
                        .rich_client
                        .update(&Packet::new(ctx.cord.rich_client.pid, Some(&activity)))?;
                }

                session.last_activity = Some(activity);
            }
        }

        Ok(())
    }
}
