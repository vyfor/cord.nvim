#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
    pub level: LogLevel,
}

use std::collections::HashMap;

use crate::messages::events::event::{EventContext, OnEvent};
use crate::protocol::msgpack::{Serialize, ValueRef};
use crate::util::logger::LogLevel;

impl OnEvent for LogEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        ctx.cord
            .logger
            .log(self.level, self.message.into(), ctx.client_id);

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
        data.insert("message", ValueRef::String(&self.message));
        data.insert("level", ValueRef::UInteger(self.level as u64));

        f("type", ValueRef::String("log"), state)?;
        f("data", ValueRef::Map(data), state)?;

        Ok(())
    }
}
