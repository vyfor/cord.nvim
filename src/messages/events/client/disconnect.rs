#[derive(Debug, Default)]
pub struct DisconnectEvent;

impl DisconnectEvent {
    pub fn on_disconnect(self) {}
}
