use core::clone::Clone;
use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::{Activity, ActivityTimestamps};
use crate::presence::packet::Packet;
use crate::util::now;

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

        let mut activity = self
            .activity
            .is_idle
            .then_some(
                ctx.cord
                    .session_manager
                    .sessions
                    .read()
                    .unwrap()
                    .iter()
                    .filter(|s| {
                        s.0 != &ctx.client_id
                            && s.1
                                .last_activity
                                .as_ref()
                                .is_some_and(|a| !a.is_idle)
                    })
                    .max_by_key(|s| s.1.last_updated)
                    .and_then(|(_, s)| s.last_activity.clone()),
            )
            .flatten()
            .unwrap_or(self.activity);

        if ctx.cord.config.shared_timestamps {
            let shared_ts = &ctx.cord.session_manager.shared_timestamp;
            let ts_ref =
                activity
                    .timestamps
                    .get_or_insert_with(|| ActivityTimestamps {
                        start: Some(shared_ts.load(Ordering::SeqCst)),
                        end: None,
                    });
            if let Some(start) = ts_ref.start {
                shared_ts.store(start, Ordering::SeqCst);
            } else {
                ts_ref.start = Some(shared_ts.load(Ordering::SeqCst));
            }
        }

        rich_client.update(&Packet::new(rich_client.pid, Some(&activity)))?;

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_last_activity(activity);
            session.last_updated = now().as_nanos();
        }

        Ok(())
    }
}
