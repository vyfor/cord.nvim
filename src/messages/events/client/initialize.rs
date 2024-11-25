use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::PluginConfig;

#[derive(Debug)]
pub struct InitializeEvent {
    pub config: PluginConfig,
}

impl OnEvent for InitializeEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord.logger.set_level(self.config.log_level);
        ctx.cord.plugin_config = Some(self.config);

        Ok(())
    }
}

impl InitializeEvent {
    pub fn new(config: PluginConfig) -> Self {
        Self { config }
    }
}
