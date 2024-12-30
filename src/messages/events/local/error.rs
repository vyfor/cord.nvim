use crate::ipc::discord::error::DiscordError;
use crate::messages::events::event::{EventContext, OnEvent};
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
                    return Err("The connection was forcibly closed".into());
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
