pub mod error;

pub use error::ErrorEvent;

use super::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum LocalEvent {
    Error(ErrorEvent),
}

impl OnEvent for LocalEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Error(e) => e.on_event(ctx),
        }
    }
}
