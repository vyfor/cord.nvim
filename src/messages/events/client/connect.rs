#[derive(Debug, Default)]
pub struct ConnectEvent;

use std::sync::atomic::Ordering;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::ReadyEvent;
use crate::protocol::msgpack::MsgPack;

impl OnEvent for ConnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if ctx.cord.rich_client.is_ready.load(Ordering::SeqCst) {
            println!("Sending ready event...");
            ctx.cord
                .pipe
                .write_to(ctx.client_id, &MsgPack::serialize(&ReadyEvent)?)?;
        }

        Ok(())
    }
}
