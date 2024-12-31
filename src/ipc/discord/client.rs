use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

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
    pub pipe: Option<std::sync::Arc<std::fs::File>>,
    #[cfg(not(target_os = "windows"))]
    pub read_pipe: Option<std::os::unix::net::UnixStream>,
    #[cfg(not(target_os = "windows"))]
    pub write_pipe: Option<std::os::unix::net::UnixStream>,
    pub pid: u32,
    pub is_ready: AtomicBool,
    pub thread_handle: Option<JoinHandle<()>>,
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
}
