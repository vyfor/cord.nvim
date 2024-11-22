use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ClientDisconnectedEvent;

impl OnEvent for ClientDisconnectedEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}
