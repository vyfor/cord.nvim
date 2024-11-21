#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
}

impl LogEvent {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn on_log(self) {}
}
