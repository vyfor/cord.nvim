#[derive(Debug)]
pub struct UpdateWorkspaceEvent {
    pub workspace: String,
}

impl UpdateWorkspaceEvent {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }

    pub fn on_update_workspace(self) {}
}
