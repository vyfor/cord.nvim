#[derive(Debug, Default)]
pub struct ReadyEvent;

impl ReadyEvent {
    pub fn on_ready(self) {}
}
