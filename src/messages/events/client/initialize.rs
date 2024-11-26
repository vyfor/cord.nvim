use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::{validate_image, PluginConfig};

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
    fn on_event(mut self, ctx: &mut EventContext) -> crate::Result<()> {
        validate_image(
            &mut self.config.editor.image,
            ctx.cord.config.is_custom_client,
        );
        ctx.cord.logger.set_level(self.config.log_level);

        if ctx.cord.session_manager.get_default_config().is_none() {
            ctx.cord
                .session_manager
                .set_default_config(self.config.clone());
        }

        ctx.cord.session_manager.create_session(ctx.client_id);
        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            session.set_config(self.config);
        }

        Ok(())
    }
}
