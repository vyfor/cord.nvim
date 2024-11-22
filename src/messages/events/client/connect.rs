#[derive(Debug, Default)]
pub struct ConnectEvent;

use crate::messages::events::event::{EventContext, OnEvent};


impl OnEvent for ConnectEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}
