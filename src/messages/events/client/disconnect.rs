use crate::messages::events::event::{EventContext, OnEvent};
use crate::presence::packet::Packet;

#[derive(Debug, Default)]
pub struct DisconnectEvent;

impl OnEvent for DisconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let mut sessions = ctx.cord.session_manager.sessions.write().unwrap();
        sessions.remove(&ctx.client_id);
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
            let rich_client = ctx.cord.rich_client.read().unwrap();
            rich_client.update(&Packet::new(
                rich_client.pid,
                session.last_activity.as_ref(),
            ))?;
        } else {
            ctx.cord.rich_client.read().unwrap().clear()?;
        }

        Ok(())
    }
}
