use std::sync::atomic::Ordering;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::activity::ActivityTimestamps;
use crate::presence::packet::Packet;
use crate::protocol::msgpack::Deserialize;

#[derive(Debug, Default)]
pub struct ClearActivityEvent {
    force: bool,
}

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let global_last_activity = ctx
            .cord
            .session_manager
            .last_activity
            .read()
            .unwrap()
            .clone();

        if self.force {
            if let Some(mut session) =
                ctx.cord.session_manager.get_session_mut(ctx.client_id)
            {
                session.last_activity = None;
            }

            if global_last_activity.is_some() {
                *ctx.cord.session_manager.last_activity.write().unwrap() = None;
                ctx.cord.rich_client.read().unwrap().clear()?;
            }
        } else {
            let mut sessions =
                ctx.cord.session_manager.sessions.write().unwrap();

            if let Some(session) = sessions.get_mut(&ctx.client_id) {
                session.last_activity = None;
            }

            let latest = sessions
                .iter()
                .filter(|s| {
                    s.0 != &ctx.client_id && s.1.last_activity.is_some()
                })
                .max_by_key(|s| {
                    (
                        s.1.last_activity.as_ref().is_some_and(|a| !a.is_idle),
                        s.1.last_updated,
                    )
                })
                .map(|(_, s)| s);

            if let Some(session) = latest {
                if let Some(mut activity) = session.last_activity.clone() {
                    if let Some(global) = &global_last_activity {
                        if global == &activity {
                            return Ok(());
                        }
                    }

                    if ctx.cord.config.shared_timestamps {
                        let shared_ts =
                            &ctx.cord.session_manager.shared_timestamp;
                        let ts_ref =
                            activity.timestamps.get_or_insert_with(|| {
                                ActivityTimestamps {
                                    start: Some(
                                        shared_ts.load(Ordering::SeqCst),
                                    ),
                                    end: None,
                                }
                            });
                        if ts_ref.start.is_none() {
                            ts_ref.start =
                                Some(shared_ts.load(Ordering::SeqCst));
                        }
                    }

                    ctx.cord
                        .session_manager
                        .last_activity
                        .write()
                        .unwrap()
                        .replace(activity.clone());

                    let rich_client = ctx.cord.rich_client.read().unwrap();
                    rich_client.update(&Packet::new(
                        rich_client.pid,
                        Some(&activity),
                    ))?;
                }
            } else if global_last_activity.is_some() {
                *ctx.cord.session_manager.last_activity.write().unwrap() = None;
                ctx.cord.rich_client.read().unwrap().clear()?;
            }
        }

        Ok(())
    }
}

impl Deserialize for ClearActivityEvent {
    fn deserialize(
        input: crate::protocol::msgpack::Value,
    ) -> crate::Result<Self> {
        let force = input.as_bool().unwrap_or_default();

        Ok(ClearActivityEvent { force })
    }
}
