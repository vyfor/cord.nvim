pub mod error;
pub mod reconnect;
pub mod reconnect_complete;

pub use error::ErrorEvent;
pub use reconnect::ReconnectEvent;
pub use reconnect_complete::ReconnectCompleteEvent;

use crate::trace;
use super::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum LocalEvent {
    Error(ErrorEvent),
    Reconnect(ReconnectEvent),
    ReconnectComplete(ReconnectCompleteEvent),
}

impl OnEvent for LocalEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Error(e) => {
                trace!(ctx.client_id, "Dispatching local error event");
                e.on_event(ctx)
            }
            Self::Reconnect(e) => {
                trace!(ctx.client_id, "Dispatching local reconnect event");
                e.on_event(ctx)
            }
            Self::ReconnectComplete(e) => {
                trace!(
                    ctx.client_id,
                    "Dispatching local reconnect complete event"
                );
                e.on_event(ctx)
            }
        }
    }
}
