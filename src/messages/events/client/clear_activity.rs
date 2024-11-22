#[derive(Debug, Default)]
pub struct ClearActivityEvent;

use crate::messages::events::event::{EventContext, OnEvent};

impl OnEvent for ClearActivityEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}
