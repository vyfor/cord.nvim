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
    fn connect(&mut self) -> crate::Result<()>;
    /// Closes the connection to Discord.
    fn close(&mut self);
    /// Start reading from Discord in a separate thread
    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()>;
    /// Write data to Discord
    fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()>;
}

impl RichClient {
    pub fn new(client_id: u64) -> Self {
        Self {
            client_id,
            #[cfg(target_os = "windows")]
            pipe: None,
            #[cfg(not(target_os = "windows"))]
            read_pipe: None,
            #[cfg(not(target_os = "windows"))]
            write_pipe: None,
            pid: std::process::id(),
            is_ready: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
            is_reconnecting: Arc::new(AtomicBool::new(false)),
        }
    }

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
    pub fn reconnect(&mut self, interval: u64, tx: Sender<Message>) {
        self.is_reconnecting.store(true, Ordering::SeqCst);
        self.close();

        std::thread::sleep(Duration::from_millis(500));

        let mut client = Self::new(self.client_id);
        while self.is_reconnecting.load(Ordering::SeqCst) {
            if client.connect().is_ok() {
                if client.handshake().is_ok() {
                    *self = client;
                    if self.start_read_thread(tx).is_err() {

                        self.is_reconnecting.store(false, Ordering::SeqCst);
                    };

                    break;
                } else {
                    client.close();
                }
            };

            std::thread::sleep(Duration::from_millis(interval));
        }

        self.is_reconnecting.store(false, Ordering::SeqCst);
    }
}
