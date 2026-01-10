pub mod log;
pub mod status_update;

pub use log::LogEvent;
pub use status_update::StatusUpdateEvent;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::trace;

#[derive(Debug)]
pub enum ServerEvent {
    Log(LogEvent),
    StatusUpdate(StatusUpdateEvent),
}

impl OnEvent for ServerEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Log(e) => e.on_event(ctx),
            Self::StatusUpdate(e) => {
                trace!(
                    ctx.client_id,
                    "Dispatching server status_update event: {}", e.status
                );
                e.on_event(ctx)
            }
        }
    }
}
