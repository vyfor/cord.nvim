#[derive(Debug, Default)]
pub struct ConnectEvent;

use std::sync::atomic::Ordering;

use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::{
        event::{EventContext, OnEvent},
        server::ReadyEvent,
    },
    msgpack::MsgPack,
};

impl OnEvent for ConnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if ctx.cord.rich_client.is_ready.load(Ordering::SeqCst) {
            ctx.cord
                .pipe
                .write_to(ctx.client_id, &MsgPack::serialize(&ReadyEvent)?)?;
        }

        Ok(())
    }
}
