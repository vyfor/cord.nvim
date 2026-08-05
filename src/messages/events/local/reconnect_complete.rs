use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::StatusUpdateEvent;
use crate::protocol::msgpack::MsgPack;
use crate::{debug, error};

#[derive(Debug)]
pub enum ReconnectStatus {
    Ok,
    Err(String),
}

impl From<crate::Result<()>> for ReconnectStatus {
    fn from(result: crate::Result<()>) -> Self {
        match result {
            Ok(()) => Self::Ok,
            Err(err) => Self::Err(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ReconnectCompleteEvent {
    pub manual: bool,
    pub status: ReconnectStatus,
}

impl ReconnectCompleteEvent {
    pub fn new(manual: bool, status: ReconnectStatus) -> Self {
        Self { manual, status }
    }
}

impl OnEvent for ReconnectCompleteEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let client_id = ctx.client_id;
        ctx.cord.reconnect_state.in_progress = false;
        ctx.cord.reconnect_state.cancel = None;

        match &self.status {
            ReconnectStatus::Ok => {
                debug!(client_id, "Reconnect successful");
                *ctx.cord.session_manager.last_activity.write().unwrap() = None;
            }
            ReconnectStatus::Err(err) => {
                debug!(client_id, "Reconnect failed: {}", err);
                ctx.cord.pipe.broadcast(&MsgPack::serialize(
                    &StatusUpdateEvent::disconnected(),
                )?)?;
                if self.manual {
                    error!(client_id, "Failed to reconnect to Discord: {}", err);
                }
            }
        }

        Ok(())
    }
}
