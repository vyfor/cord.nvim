use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::BatchLogEvent;
use crate::trace;

#[derive(Debug, Default)]
pub struct ConnectEvent;

impl OnEvent for ConnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        trace!(ctx.client_id, "Processing connect event");

        let mut logs = Vec::new();
        while let Some(log) = ctx.cord.log_buffer.pop_front() {
            logs.push(log);
        }

        if !logs.is_empty() {
            BatchLogEvent { logs }.on_event(ctx)?;
        }

        Ok(())
    }
}
