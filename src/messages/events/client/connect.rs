use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct ConnectEvent;

impl OnEvent for ConnectEvent {
    fn on_event(self, _ctx: &mut EventContext) -> crate::Result<()> {
        Ok(())
    }
}
