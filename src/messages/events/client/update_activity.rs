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

        if let Some(session) = ctx.cord.session_manager.get_session(ctx.client_id) {
            if let Some(config) = session.get_config() {
                ctx.cord.rich_client.update(&Packet::new(
                    ctx.cord.rich_client.pid,
                    Some(self.context.build(config)),
                ))?;
            }
        }

        Ok(())
    }
}
