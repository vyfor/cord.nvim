pub mod client_disconnected;
pub mod error;

pub use client_disconnected::ClientDisconnectedEvent;
pub use error::ErrorEvent;

#[derive(Debug)]
pub enum LocalEvent {
    ClientDisconnected(ClientDisconnectedEvent),
    Error(ErrorEvent),
}

impl LocalEvent {
    pub fn on_event(self) {
        match self {
            LocalEvent::ClientDisconnected(event) => event.on_client_disconnected(),
            LocalEvent::Error(event) => event.on_error(),
        }
    }
}
