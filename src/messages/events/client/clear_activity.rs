use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::packet::Packet;
use crate::protocol::msgpack::Deserialize;

#[derive(Debug, Default)]
pub struct ClearActivityEvent {
    force: bool,
}

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if self.force {
            ctx.cord.rich_client.clear()?;
        } else {
            let sessions = ctx.cord.session_manager.sessions.read().unwrap();
            let latest = sessions
                .iter()
                .filter(|s| s.1.last_activity.is_some())
                .max_by_key(|s| {
                    (
                        s.1.last_activity.as_ref().is_some_and(|a| !a.is_idle),
                        s.1.last_updated,
                    )
                });

            if let Some((_, session)) = latest {
                ctx.cord.rich_client.update(&Packet::new(
                    ctx.cord.rich_client.pid,
                    session.last_activity.as_ref(),
                ))?;
            } else {
                ctx.cord.rich_client.clear()?;
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
