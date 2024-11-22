use crate::messages::events::event::{EventContext, OnEvent};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct ErrorEvent {
    pub error: Error,
}

impl ErrorEvent {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

impl OnEvent for ErrorEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}
