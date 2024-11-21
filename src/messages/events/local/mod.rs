pub mod client_disconnected;
pub mod error;

pub use client_disconnected::ClientDisconnectedEvent;
pub use error::ErrorEvent;

#[derive(Debug)]
pub enum LocalEvent {
    ClientDisconnected(ClientDisconnectedEvent),
    Error(ErrorEvent),
}
