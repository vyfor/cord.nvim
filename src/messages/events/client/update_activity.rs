use core::clone::Clone;
use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::{Activity, ActivityTimestamps};
use crate::protocol::msgpack::Deserialize;
use crate::util::now;
use crate::{debug, trace};

#[derive(Debug)]
pub struct UpdateActivityEvent {
    activity: Activity,
    force: bool,
}

impl OnEvent for UpdateActivityEvent {
    // if new activity is idle, set the most recent activity available, if not, display the new activity
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        trace!(ctx.client_id, "Processing update_activity event, force={}", self.force);
        
        if !ctx
            .cord
            .activity_manager
            .client
            .read()
            .unwrap()
            .is_ready
            .load(Ordering::SeqCst)
        {
            debug!(ctx.client_id, "Ignoring activity update: Discord not ready");
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

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_last_activity(activity.clone());
            session.last_updated = now().as_nanos();
        }

        let should_update = {
            let mut last_activity =
                ctx.cord.session_manager.last_activity.write().unwrap();

            if let Some(global_last_activity) = last_activity.as_ref() {
                if !self.force && global_last_activity == &activity {
                    trace!(ctx.client_id, "Skipping activity update: no change");
                    false
                } else {
                    *last_activity = Some(activity.clone());
                    true
                }
            } else {
                *last_activity = Some(activity.clone());
                true
            }
        };

        if should_update {
            debug!(ctx.client_id, "Updating activity: is_idle={}", activity.is_idle);
            ctx.cord.activity_manager.update(activity)?;
        }

        Ok(())
    }
}

impl Deserialize for UpdateActivityEvent {
    fn deserialize(
        input: crate::protocol::msgpack::Value,
    ) -> crate::Result<Self> {
        let mut map =
            input.take_map().ok_or("Invalid update activity event")?;
        let activity = Activity::deserialize(
            map.remove("activity")
                .ok_or("Missing or invalid 'activity' field")?,
        )?;
        let force = map
            .remove("force")
            .and_then(|v| v.as_bool())
            .unwrap_or_default();

        Ok(UpdateActivityEvent { activity, force })
    }
}
