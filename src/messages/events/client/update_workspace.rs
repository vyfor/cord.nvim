use std::borrow::Borrow;

use crate::{
    messages::events::event::{EventContext, OnEvent},
    types,
    util::utils,
};

#[derive(Debug)]
pub struct UpdateWorkspaceEvent {
    pub workspace: String,
}

impl UpdateWorkspaceEvent {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }
}

impl OnEvent for UpdateWorkspaceEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(config) = &mut ctx.cord.plugin_config {
            let workspace = utils::find_workspace(&self.workspace);
            if let Some(filename) = workspace.file_name() {
                let filename = filename.to_string_lossy();
                config.workspace = filename.to_string();

                types::config::validate_buttons(&mut config.buttons, filename.borrow());

                // todo: check for workspace blacklist
            }
        }

        Ok(())
    }
}
