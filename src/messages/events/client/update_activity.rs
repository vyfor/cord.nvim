use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::ActivityContext;
use crate::presence::types::Packet;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    pub context: ActivityContext,
}

impl UpdateActivityEvent {
    pub fn new(context: ActivityContext) -> Self {
        Self { context }
    }
}

impl OnEvent for UpdateActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(config) = &mut ctx.cord.plugin_config {
            if !ctx.cord.rich_client.is_ready.load(Ordering::SeqCst) {
                return Ok(());
            }
            ctx.cord.rich_client.update(&Packet::new(
                ctx.cord.rich_client.pid,
                Some(self.context.build(config)),
            ))?;
        }

        Ok(())
    }
}
