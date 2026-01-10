use std::collections::HashMap;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::LogEvent;
use crate::protocol::msgpack::{MsgPack, Serialize, ValueRef};

#[derive(Debug)]
pub struct BatchLogEvent {
    pub logs: Vec<LogEvent>,
}

impl OnEvent for BatchLogEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let data = MsgPack::serialize(&self)?;
        ctx.cord.pipe.write_to(ctx.client_id, &data)?;
        Ok(())
    }
}

impl Serialize for BatchLogEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        let mut logs = Vec::new();

        for log in &self.logs {
            let mut data = HashMap::new();
            data.insert("message", ValueRef::Str(&log.message));
            data.insert("level", ValueRef::UInteger(log.level as u64));
            logs.push(ValueRef::Map(data));
        }

        f("type", ValueRef::Str("log_batch"), state)?;
        f("data", ValueRef::Array(logs), state)?;

        Ok(())
    }
}
