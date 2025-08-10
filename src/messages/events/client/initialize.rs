use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::PluginConfig;
use crate::util::now;

#[derive(Debug)]
pub struct InitializeEvent {
    config: PluginConfig,
}

impl InitializeEvent {
    pub fn new(config: PluginConfig) -> Self {
        Self { config }
    }
}

impl OnEvent for InitializeEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(logger) = Arc::get_mut(&mut ctx.cord.logger) {
            logger.set_level(self.config.log_level);
        }

        ctx.cord.config.shared_timestamps = self.config.timestamp.shared;
        if self.config.timestamp.shared {
            let _ = ctx.cord.session_manager.shared_timestamp.compare_exchange(
                0,
                now().as_secs(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
        }

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_config(self.config);
        }

        Ok(())
    }
}
