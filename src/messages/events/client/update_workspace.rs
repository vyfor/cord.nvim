use std::borrow::Borrow;

use crate::{
    messages::events::event::{EventContext, OnEvent},
    types::config,
    util::utils,
};

#[derive(Debug)]
pub struct UpdateWorkspaceEvent {
    workspace: String,
}

impl UpdateWorkspaceEvent {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }
}

impl OnEvent for UpdateWorkspaceEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let workspace = utils::find_workspace(&self.workspace);
        
        if let Some(mut session) = ctx.cord.session_manager.get_session_mut(ctx.client_id) {
            if let Some(filename) = workspace.file_name() {
                let filename = filename.to_string_lossy();
                session.set_workspace(filename.to_string());
                
                if let Some(config) = session.get_config() {
                    let mut buttons = config.buttons.clone();
                    config::validate_buttons(&mut buttons, filename.borrow());
                }
            }
        }

        Ok(())
    }
}
