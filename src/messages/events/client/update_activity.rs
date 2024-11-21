use crate::presence::activity::ActivityContext;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    pub context: ActivityContext,
}

impl UpdateActivityEvent {
    pub fn new(context: ActivityContext) -> Self {
        Self { context }
    }

    pub fn on_update_activity(self) {}
}
