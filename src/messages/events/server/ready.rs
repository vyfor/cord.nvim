use std::sync::atomic::Ordering;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::EventContext;
use crate::{
    json::{
        serialize::{SValue, Serialize},
        Json,
    },
    messages::events::event::OnEvent,
};

#[derive(Debug, Default)]
pub struct ReadyEvent;

impl OnEvent for ReadyEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if !ctx.cord.rich_client.is_ready.swap(true, Ordering::SeqCst) {
            ctx.cord
                .pipe
                .broadcast(Json::serialize(&self)?.as_bytes())?;
        }

        Ok(())
    }
}

impl Serialize for ReadyEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::json::serialize::SerializeFn<'a>,
        state: &mut crate::json::serialize::SerializeState,
    ) -> crate::Result<()> {
        f("type", SValue::String("ready"), state)?;

        Ok(())
    }
}
