use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::{validate_image, PluginConfig};

#[derive(Debug)]
pub struct InitializeEvent {
    pub config: PluginConfig,
}

impl OnEvent for InitializeEvent {
    fn on_event(mut self, ctx: &mut EventContext) -> crate::Result<()> {
        validate_image(
            &mut self.config.editor_image,
            ctx.cord.config.is_custom_client,
        );
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
