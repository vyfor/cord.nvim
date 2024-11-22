#[derive(Debug, Default)]
pub struct ClearActivityEvent;

use crate::messages::events::event::{EventContext, OnEvent};

impl ClearActivityEvent {
    pub fn on_clear_activity(self) {}
}

impl OnEvent for ClearActivityEvent {
    fn on_event(self, _ctx: &EventContext) {}
}
