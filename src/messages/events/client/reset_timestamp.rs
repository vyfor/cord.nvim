use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ResetTimestampEvent;

impl OnEvent for ResetTimestampEvent {
    fn on_event(self, _ctx: &EventContext) {}
}
