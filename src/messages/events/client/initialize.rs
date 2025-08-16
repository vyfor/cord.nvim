use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::error::CordErrorKind;
use crate::ipc::discord::client::Connection;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::types::config::PluginConfig;
use crate::util::now;

#[derive(Debug)]
pub struct InitializeEvent {
    config: PluginConfig,
}

impl InitializeEvent {
    pub fn new(config: PluginConfig) -> Self {
        Self { config }
    }
}

impl OnEvent for InitializeEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        if let Some(logger) = Arc::get_mut(&mut ctx.cord.logger) {
            logger.set_level(self.config.log_level);
        }

        ctx.cord.config.shared_timestamps = self.config.timestamp.shared;
        if self.config.timestamp.shared {
            let _ = ctx.cord.session_manager.shared_timestamp.compare_exchange(
                0,
                now().as_secs(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
        }

        let rich_client = &ctx.cord.rich_client;
        let mut client = rich_client.write().unwrap();
        if !self.config.advanced.discord.pipe_paths.is_empty()
            && client.pipe_paths.is_empty()
        {
            client.pipe_paths = self.config.advanced.discord.pipe_paths.clone();
        }

        let config = &ctx.cord.config;
        if !client.is_ready.load(Ordering::SeqCst) && client.connect().is_err()
        {
            if config.reconnect_interval > 0 && config.initial_reconnect {
                drop(client);
                let client_clone = rich_client.clone();
                let tx = ctx.cord.tx.clone();

                let reconnect_interval = config.reconnect_interval;
                std::thread::spawn(move || {
                    let mut client = client_clone.write().unwrap();
                    client.reconnect(reconnect_interval, tx.clone());
                });
            } else {
                return Err(crate::error::CordError::new(
                    CordErrorKind::Io,
                    "Failed to connect to Discord",
                ));
            }
        } else {
            client.handshake()?;
            client.start_read_thread(ctx.cord.tx.clone())?;
        }

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_config(self.config);
        }

        Ok(())
    }
}
