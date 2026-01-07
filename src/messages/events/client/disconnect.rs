use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug, Default)]
pub struct DisconnectEvent;

impl OnEvent for DisconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let mut sessions = ctx.cord.session_manager.sessions.write().unwrap();
        sessions.remove(&ctx.client_id);
        if sessions.is_empty() {
            ctx.cord.activity_manager.clear()?;
            ctx.cord
                .session_manager
                .last_activity
                .write()
                .unwrap()
                .take();
            ctx.cord
                .session_manager
                .shared_timestamp
                .store(0, Ordering::SeqCst);
            return Ok(());
        }

        if ctx
            .cord
            .session_manager
            .last_activity
            .read()
            .unwrap()
            .is_none()
        {
            return Ok(());
        }

        let latest = sessions
            .iter()
            .filter(|s| s.1.last_activity.is_some())
            .max_by_key(|s| {
                (
                    s.1.last_activity.as_ref().is_some_and(|a| !a.is_idle),
                    s.1.last_updated,
                )
            })
            .map(|(_, s)| s);

        if let Some(session) = latest {
            let activity = session.last_activity.as_ref().unwrap();

            {
                let mut last_activity =
                    ctx.cord.session_manager.last_activity.write().unwrap();

                if let Some(global_last_activity) = last_activity.as_ref() {
                    if global_last_activity == activity {
                        return Ok(());
                    }
                }

                *last_activity = Some(activity.clone());
            }

            ctx.cord.activity_manager.update(activity.clone())?;
        }

        Ok(())
    }
}
