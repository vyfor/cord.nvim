pub mod log;
pub mod ready;

pub use log::LogEvent;
pub use ready::ReadyEvent;

use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
}

impl OnEvent for ServerEvent {
    fn on_event(self, ctx: &EventContext) {
        match self {
            Self::Ready(e) => e.on_event(ctx),
            Self::Log(e) => e.on_event(ctx),
        }
    }
}
