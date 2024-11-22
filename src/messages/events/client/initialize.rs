use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::Config;

#[derive(Debug)]
pub struct InitializeEvent {
    pub config: Config,
}

impl OnEvent for InitializeEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}

impl InitializeEvent {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn on_initialize(self) {}
}
