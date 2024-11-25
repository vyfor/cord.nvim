#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
}

use crate::{
    ipc::pipe::PipeServerImpl,
    messages::events::event::{EventContext, OnEvent},
};

impl OnEvent for LogEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match ctx.client_id {
            0 => ctx.cord.pipe.broadcast(self.message.as_bytes()),
            _ => ctx
                .cord
                .pipe
                .write_to(ctx.client_id, self.message.as_bytes()),
        }?;

        Ok(())
    }
}

impl LogEvent {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
