use crate::ipc::discord::error::DiscordError;
use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::local::ReconnectEvent;
use crate::messages::events::server::StatusUpdateEvent;
use crate::protocol::msgpack::MsgPack;
use crate::{debug, error, local_event};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct ErrorEvent {
    pub error: Error,
}

impl ErrorEvent {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

impl OnEvent for ErrorEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(discord_error) = self.error.downcast_ref::<DiscordError>() {
            match discord_error {
                DiscordError::InvalidClientId(id) => {
                    return Err(
                        format!("'{}' is not a valid client ID", id).into()
                    );
                }
                DiscordError::ConnectionClosed => {
                    if ctx.cord.reconnect_state.in_progress {
                        debug!(
                            "Discord closed the connection during reconnect"
                        );
                        return Ok(());
                    }

                    let reconnect_interval = ctx.cord.config.reconnect_interval;
                    if reconnect_interval == 0 {
                        debug!(
                            "connection closed, reconnect_interval=0, returning error"
                        );
                        return Err("Discord closed the connection".into());
                    }

                    debug!(
                        "connection closed, reconnect_interval={}, scheduling reconnect",
                        reconnect_interval
                    );
                    let _ = ctx.cord.tx.send(local_event!(
                        0,
                        Reconnect,
                        ReconnectEvent::new(false)
                    ));

                    debug!("Discord closed the connection");

                    return Ok(());
                }
                _ => {
                    ctx.cord.pipe.broadcast(&MsgPack::serialize(
                        &StatusUpdateEvent::disconnected(),
                    )?)?;
                    error!(ctx.client_id, "{}", self.error);

                    return Ok(());
                }
            }
        }
        error!(ctx.client_id, "{}", self.error);

        Ok(())
    }
}
