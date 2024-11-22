#[derive(Debug, Default)]
pub struct ConnectEvent;

use std::sync::atomic::Ordering;

use crate::{
    ipc::pipe::PipeServerImpl,
    json::Json,
    messages::events::{
        event::{EventContext, OnEvent},
        server::ReadyEvent,
    },
};

impl OnEvent for ConnectEvent {
    fn on_event(self, ctx: &EventContext) -> crate::Result<()> {
        if ctx.rich_client.is_ready.load(Ordering::SeqCst) {
            ctx.pipe
                .write_to(ctx.client_id, Json::serialize(&ReadyEvent)?.as_bytes())?;
        }

        Ok(())
    }
}
