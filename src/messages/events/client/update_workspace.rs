use crate::messages::events::event::{EventContext, OnEvent};

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
    fn on_event(self, _ctx: &EventContext) {}
}
