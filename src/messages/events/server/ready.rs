#[derive(Debug, Default)]
pub struct ReadyEvent;

impl ReadyEvent {
    pub fn on_ready(self) -> Option<(u32, String)> {
        Some((0, "Ready".to_string()))
    }
}
