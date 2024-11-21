#[derive(Debug, Default)]
pub struct ResetTimestampEvent;

impl ResetTimestampEvent {
    pub fn on_timestamp_reset(self) {}
}
