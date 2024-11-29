pub mod log;
pub mod ready;
pub mod workspace_blacklisted;

pub use log::LogEvent;
pub use ready::ReadyEvent;
pub use workspace_blacklisted::WorkspaceBlacklistedEvent;

use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
    WorkspaceBlacklisted(WorkspaceBlacklistedEvent),
}

impl OnEvent for ServerEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Ready(e) => e.on_event(ctx),
            Self::Log(e) => e.on_event(ctx),
            Self::WorkspaceBlacklisted(e) => e.on_event(ctx),
        }
    }
}
