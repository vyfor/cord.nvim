use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::Activity;
use crate::presence::packet::Packet;

#[derive(Debug)]
pub struct UpdateActivityEvent {
    activity: Activity,
}

impl UpdateActivityEvent {
    pub fn new(activity: Activity) -> Self {
        Self { activity }
    }
}

impl OnEvent for UpdateActivityEvent {
    // if new activity is idle, set the most recent activity available, if not, display the new activity
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let rich_client = ctx.cord.rich_client.read().unwrap();
        if !rich_client.is_ready.load(Ordering::SeqCst) {
            return Ok(());
        }

        let mut sessions = ctx.cord.session_manager.sessions.write().unwrap();
        let is_idle = self.activity.is_idle;
        let activity = is_idle
            .then_some(
                sessions
                    .iter()
                    .filter(|s| {
                        s.0 != &ctx.client_id
                            && s.1
                                .last_activity
                                .as_ref()
                                .is_some_and(|a| !a.is_idle)
                    })
                    .max_by_key(|s| s.1.last_updated)
                    .and_then(|(_, s)| s.last_activity.as_ref()),
            )
            .flatten()
            .unwrap_or(&self.activity);

        rich_client.update(&Packet::new(rich_client.pid, Some(activity)))?;

        if let Some(session) = sessions.get_mut(&ctx.client_id) {
            session.set_last_activity(self.activity);
            session.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap() // todo: reset all timestamps if this fails
                .as_nanos();
        }

        Ok(())
    }
}
