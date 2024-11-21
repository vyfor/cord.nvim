#[derive(Debug, Default)]
pub struct ClientDisconnectedEvent;

impl ClientDisconnectedEvent {
    pub fn on_client_disconnected(self) {}
}
