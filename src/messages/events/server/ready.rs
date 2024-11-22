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
    fn on_event(self, ctx: &EventContext) {
        if let Ok(json) = Json::serialize(&self) {
            let _ = ctx.pipe.broadcast(json.as_bytes());
        }
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
