use crate::{
    messages::events::event::{EventContext, OnEvent},
    presence::packet::Packet,
};

#[derive(Debug, Default)]
pub struct ClearActivityEvent;

impl OnEvent for ClearActivityEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let sessions = ctx.cord.session_manager.sessions.read().unwrap();
        let latest = sessions
            .iter()
            .filter(|s| s.1.last_activity.is_some())
            .max_by_key(|s| s.1.last_updated);

        if let Some((_, session)) = latest {
            ctx.cord.rich_client.update(&Packet::new(
                ctx.cord.rich_client.pid,
                session.last_activity.as_ref(),
            ))?;
        } else {
            ctx.cord.rich_client.clear()?;
        }

        Ok(())
    }
}
