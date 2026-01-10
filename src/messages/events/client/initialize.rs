use std::sync::atomic::Ordering;

use crate::debug;
use crate::error::CordErrorKind;
use crate::ipc::discord::client::Connection;
use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::StatusUpdateEvent;
use crate::messages::events::server::status_update::Status;
use crate::protocol::msgpack::MsgPack;
use crate::types::config::PluginConfig;
use crate::util::{logger, now};

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
        debug!(ctx.client_id, "Processing initialize event");
        if let Some(logger) = logger::LOGGER.get() {
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

        ctx.cord
            .activity_manager
            .set_config(self.config.advanced.discord.sync.clone());

        let rich_client = &ctx.cord.activity_manager.client;
        let mut client = rich_client.write().unwrap();
        if !self.config.advanced.discord.pipe_paths.is_empty()
            && client.pipe_paths.is_empty()
        {
            debug!(ctx.client_id, "Setting custom Discord pipe paths");
            client.pipe_paths = self.config.advanced.discord.pipe_paths.clone();
        }

        let config = &ctx.cord.config;
        let is_ready = client.is_ready.load(Ordering::SeqCst);
        let has_thread = client.thread_handle.is_some();
        if !has_thread && !is_ready {
            debug!(ctx.client_id, "Initiating Discord connection");
            client.status = Status::Connecting;
            ctx.cord.pipe.broadcast(&MsgPack::serialize(
                &StatusUpdateEvent::connecting(),
            )?)?;
            if client.connect().is_err() {
                if config.reconnect_interval > 0 && config.initial_reconnect {
                    debug!(
                        ctx.client_id,
                        "Connection failed, scheduling reconnect"
                    );
                    drop(client);
                    let client_clone = rich_client.clone();
                    let tx = ctx.cord.tx.clone();

                    let reconnect_interval = config.reconnect_interval;
                    std::thread::spawn(move || {
                        let mut client = client_clone.write().unwrap();
                        client.reconnect(reconnect_interval, tx.clone());
                    });
                } else {
                    debug!(
                        ctx.client_id,
                        "Connection failed, no reconnect configured"
                    );
                    return Err(crate::error::CordError::new(
                        CordErrorKind::Io,
                        "Failed to connect to Discord",
                    ));
                }
            } else {
                debug!(ctx.client_id, "Successfully connected to Discord");
                client.status = Status::Connected;
                ctx.cord.pipe.broadcast(&MsgPack::serialize(
                    &StatusUpdateEvent::connected(),
                )?)?;
                client.handshake()?;
                client.start_read_thread(ctx.cord.tx.clone())?;
            }
        } else {
            if is_ready {
                debug!(ctx.client_id, "Discord already ready");
                client.status = Status::Ready;
            }

            ctx.cord.pipe.broadcast(&MsgPack::serialize(
                &StatusUpdateEvent::new(client.status),
            )?)?;
        }

        if let Some(mut session) =
            ctx.cord.session_manager.get_session_mut(ctx.client_id)
        {
            session.set_config(self.config);
        }

        Ok(())
    }
}
