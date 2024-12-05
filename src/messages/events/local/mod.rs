pub mod error;
pub mod shutdown;

pub use error::ErrorEvent;
pub use shutdown::ShutdownEvent;

use super::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum LocalEvent {
    Error(ErrorEvent),
    Shutdown(ShutdownEvent),
}

impl OnEvent for LocalEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Error(e) => e.on_event(ctx),
            Self::Shutdown(e) => e.on_event(ctx),
        }
    }
}
