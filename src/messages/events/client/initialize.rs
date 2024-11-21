use crate::types::Config;

#[derive(Debug)]
pub struct InitializeEvent {
    pub config: Config,
}

impl InitializeEvent {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn on_initialize(self) {}
}
