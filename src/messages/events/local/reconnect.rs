use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::ipc::discord::client::Connection;
use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::StatusUpdateEvent;
use crate::messages::events::local::ReconnectCompleteEvent;
use crate::protocol::msgpack::MsgPack;
use crate::{debug, local_event, trace};

#[derive(Debug)]
pub struct ReconnectEvent {
    pub manual: bool,
}

impl ReconnectEvent {
    pub fn new(manual: bool) -> Self {
        Self { manual }
    }
}

impl OnEvent for ReconnectEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        let client_id = ctx.client_id;

        if ctx.cord.reconnect_state.in_progress {
            if self.manual {
                debug!(
                    client_id,
                    "Cancelling in-flight reconnect request and restarting"
                );
                ctx.cord.reconnect_state.cancel();
            } else {
                trace!(
                    client_id,
                    "Dropping reconnect request as another one is already in progress"
                );
                return Ok(());
            }
        }

        let rich_client = ctx.cord.activity_manager.client.clone();
        let cancel = Arc::new(AtomicBool::new(false));
        ctx.cord.reconnect_state.in_progress = true;
        ctx.cord.reconnect_state.cancel = Some(cancel.clone());

        ctx.cord.pipe.broadcast(&MsgPack::serialize(
            &StatusUpdateEvent::disconnected(),
        )?)?;
        ctx.cord.pipe.broadcast(&MsgPack::serialize(
            &StatusUpdateEvent::connecting(),
        )?)?;

        let interval = ctx.cord.config.reconnect_interval;
        let tx = ctx.cord.tx.clone();
        let manual = self.manual;

        debug!(client_id, "Spawning reconnect worker (manual={})", manual);

        std::thread::spawn(move || {
            let mut client = rich_client.write().unwrap();
            client.is_reconnecting = true;
            client.close();

            let mut result = Ok(());
            loop {
                if cancel.load(Ordering::SeqCst) {
                    debug!(client_id, "Reconnect loop cancelled");
                    result = Err("Reconnect cancelled".into());
                    break;
                }

                std::thread::sleep(Duration::from_millis(500));

                let mut rich_client =
                    crate::ipc::discord::client::RichClient::new(
                        client.client_id,
                        client.pipe_paths.clone(),
                    );

                match rich_client.connect() {
                    Ok(()) => match rich_client.handshake() {
                        Ok(()) => {
                            if let Err(e) =
                                rich_client.start_read_thread(tx.clone())
                            {
                                debug!(
                                    client_id,
                                    "Reconnect: start_read_thread failed: {}", e
                                );
                                rich_client.close();
                                if !manual && interval > 0 {
                                    std::thread::sleep(Duration::from_millis(
                                        interval,
                                    ));
                                    continue;
                                }
                                result = Err(e);
                                break;
                            }
                            debug!(client_id, "Reconnected to Discord");
                            *client = rich_client;
                            break;
                        }
                        Err(e) => {
                            debug!(client_id, "Reconnect: handshake failed: {}", e);
                            rich_client.close();
                            if !manual && interval > 0 {
                                std::thread::sleep(Duration::from_millis(
                                    interval,
                                ));
                                continue;
                            }
                            result = Err(e);
                            break;
                        }
                    },
                    Err(e) => {
                        debug!(client_id, "Reconnect: connect failed: {}", e);
                        if !manual && interval > 0 {
                            std::thread::sleep(Duration::from_millis(interval));
                            continue;
                        }
                        result = Err(e);
                        break;
                    }
                }
            }

            client.is_reconnecting = false;
            let _ = tx.send(local_event!(
                client_id,
                ReconnectComplete,
                ReconnectCompleteEvent::new(manual, result.into())
            ));
        });

        Ok(())
    }
}
