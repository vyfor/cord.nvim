#[derive(Debug, Default)]
pub struct ConnectEvent;

use crate::messages::events::event::{EventContext, OnEvent};

impl ConnectEvent {
    pub fn on_connect(self) {}
}

impl OnEvent for ConnectEvent {
    fn on_event(self, _ctx: &EventContext) {}
}
