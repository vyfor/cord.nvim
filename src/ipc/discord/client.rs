use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::messages::message::Message;
use crate::presence::packet::Packet;
use crate::protocol::json::Json;

/// Manages the connection to Discord for sending and receiving data.
///
/// # Fields
/// * `client_id`: The ID of the Discord client.
/// * `pipe`: The communication pipe (platform-specific).
/// * `pid`: Process ID.
/// * `is_ready`: Indicates if the client is ready.
pub struct RichClient {
    pub client_id: u64,
    #[cfg(target_os = "windows")]
    pub pipe: Option<Arc<std::fs::File>>,
    #[cfg(not(target_os = "windows"))]
    pub read_pipe: Option<std::os::unix::net::UnixStream>,
    #[cfg(not(target_os = "windows"))]
    pub write_pipe: Option<std::os::unix::net::UnixStream>,
    pub pid: u32,
    pub is_ready: Arc<AtomicBool>,
    pub thread_handle: Option<JoinHandle<()>>,
    pub is_reconnecting: Arc<AtomicBool>,
}

/// Defines methods for connecting and closing the client.
pub trait Connection {
    /// Connects to Discord using the given client ID.
    fn connect(client_id: u64) -> crate::Result<RichClient>;
    /// Closes the connection to Discord.
    fn close(&mut self);
    /// Start reading from Discord in a separate thread
    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()>;
    /// Write data to Discord
    fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()>;
}

impl RichClient {
    /// Establishes a connection with Discord.
    pub fn handshake(&self) -> crate::Result<()> {
        self.write(
            0,
            Some(
                format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id)
                    .as_bytes(),
            ),
        )
    }

    /// Updates the client's rich presence.
    pub fn update(&self, packet: &Packet) -> crate::Result<()> {
        let encoded = Json::serialize(packet)?;
        match self.write(1, Some(encoded.as_bytes())) {
            Err(_) => Err("The connection to Discord was lost".into()),
            _ => Ok(()),
        }
    }

    /// Clears the current rich presence.
    pub fn clear(&self) -> crate::Result<()> {
        let packet = Packet::empty();
        let encoded = Json::serialize(&packet)?;

        match self.write(1, Some(encoded.as_bytes())) {
            Err(_) => Err("The connection to Discord was lost".into()),
            _ => Ok(()),
        }
    }

    /// Reconnects to Discord with exponential backoff.
    pub fn reconnect(
        &mut self,
        initial_interval: u64,
        tx: Sender<Message>,
    ) -> crate::Result<()> {
        self.is_reconnecting.store(true, Ordering::SeqCst);
        self.close();

        std::thread::sleep(Duration::from_millis(500));

        while self.is_reconnecting.load(Ordering::SeqCst) {
            if let Ok(mut client) = Self::connect(self.client_id) {
                if client.handshake().is_ok() {
                    *self = client;
                    if let Err(e) = self.start_read_thread(tx) {
                        self.is_reconnecting.store(false, Ordering::SeqCst);
                        return Err(e);
                    };

                    break;
                } else {
                    client.close();
                }
            };

            std::thread::sleep(Duration::from_millis(initial_interval));
        }

        self.is_reconnecting.store(false, Ordering::SeqCst);

        Ok(())
    }
}
