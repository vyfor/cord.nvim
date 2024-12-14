pub mod log;
pub mod ready;
pub mod initialize;

pub use log::LogEvent;
pub use ready::ReadyEvent;
pub use initialize::InitializeEvent;

use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
    Initialize(InitializeEvent),
}

impl OnEvent for ServerEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Ready(e) => e.on_event(ctx),
            Self::Log(e) => e.on_event(ctx),
            Self::Initialize(e) => e.on_event(ctx),
        }
    }
}
