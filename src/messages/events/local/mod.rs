pub mod client_disconnected;
pub mod error;

pub use client_disconnected::ClientDisconnectedEvent;
pub use error::ErrorEvent;

use super::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum LocalEvent {
    ClientDisconnected(ClientDisconnectedEvent),
    Error(ErrorEvent),
}

impl OnEvent for LocalEvent {
    fn on_event(self, ctx: &EventContext) {
        match self {
            Self::ClientDisconnected(e) => e.on_event(ctx),
            Self::Error(e) => e.on_event(ctx),
        }
    }
}
