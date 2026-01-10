#[derive(Debug, Clone)]
pub struct LogEvent {
    pub message: String,
    pub level: LogLevel,
}

use std::collections::HashMap;

use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::protocol::msgpack::{MsgPack, Serialize, ValueRef};
use crate::util::logger::LogLevel;

impl OnEvent for LogEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if ctx.cord.session_manager.sessions.read().unwrap().is_empty() {
            if ctx.cord.log_buffer.len() >= 100 {
                ctx.cord.log_buffer.pop_front();
            }

            ctx.cord.log_buffer.push_back(self);
        } else {
            let data = MsgPack::serialize(&self)?;
            match ctx.client_id {
                0 => ctx.cord.pipe.broadcast(&data)?,
                _ => ctx.cord.pipe.write_to(ctx.client_id, &data)?,
            }
        }

        Ok(())
    }
}

impl LogEvent {
    pub fn new(message: String, level: LogLevel) -> Self {
        Self { message, level }
    }
}

impl Serialize for LogEvent {
    fn serialize<'a>(
        &'a self,
        f: crate::protocol::msgpack::SerializeFn<'a>,
        state: &mut crate::protocol::msgpack::SerializeState,
    ) -> crate::Result<()> {
        let mut data = HashMap::new();
        data.insert("message", ValueRef::Str(&self.message));
        data.insert("level", ValueRef::UInteger(self.level as u64));

        f("type", ValueRef::Str("log"), state)?;
        f("data", ValueRef::Map(data), state)?;

        Ok(())
    }
}
