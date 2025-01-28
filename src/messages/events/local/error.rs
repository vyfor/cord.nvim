use crate::ipc::discord::error::DiscordError;
use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::DisconnectEvent;
use crate::protocol::msgpack::MsgPack;
use crate::util::logger::LogLevel;

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
                    let reconnect_interval = ctx.cord.config.reconnect_interval;
                    if reconnect_interval == 0 {
                        return Err("Discord closed the connection".into());
                    }

                    ctx.cord
                        .pipe
                        .broadcast(&MsgPack::serialize(&DisconnectEvent)?)?;

                    let rich_client = ctx.cord.rich_client.clone();
                    let tx = ctx.cord.tx.clone();
                    std::thread::spawn(move || {
                        rich_client
                            .write()
                            .unwrap()
                            .reconnect(reconnect_interval, tx);
                    });

                    ctx.cord.logger.log(
                        LogLevel::Debug,
                        "Discord closed the connection".into(),
                        0,
                    );

                    return Ok(());
                }
                _ => (),
            }
        }
        ctx.cord.logger.log(
            LogLevel::Error,
            self.error.to_string().into(),
            ctx.client_id,
        );

        Ok(())
    }
}
