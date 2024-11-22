use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::ActivityContext;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    pub context: ActivityContext,
}

impl UpdateActivityEvent {
    pub fn new(context: ActivityContext) -> Self {
        Self { context }
    }
}

impl OnEvent for UpdateActivityEvent {
    fn on_event(self, _ctx: &mut EventContext) -> crate::Result<()> {
        Ok(())
    }
}
