use std::sync::Arc;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::PluginConfig;

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

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_config(self.config);
        }

        Ok(())
    }
}
