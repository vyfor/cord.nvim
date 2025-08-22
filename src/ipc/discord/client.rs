use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::ipc::discord::error::DiscordError;
use crate::messages::events::server::status_update::Status;
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
    pub pipe_paths: Vec<String>,
    #[cfg(target_os = "windows")]
    pub pipe: Option<Arc<std::fs::File>>,
    #[cfg(not(target_os = "windows"))]
    pub read_pipe: Option<std::os::unix::net::UnixStream>,
    #[cfg(not(target_os = "windows"))]
    pub write_pipe: Option<std::os::unix::net::UnixStream>,
    pub pid: u32,
    pub is_ready: Arc<AtomicBool>,
    pub thread_handle: Option<JoinHandle<()>>,
    pub is_reconnecting: bool,
    /// Managed externally.
    pub status: Status,
}

/// Defines methods for connecting and closing the client.
pub trait Connection {
    /// Connects to the given pipe.
    fn try_connect(&mut self, pipe: &str) -> crate::Result<bool>;
    /// Closes the connection to the pipe.
    fn close(&mut self);
    /// Start reading from the pipe in a separate thread.
    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()>;
    /// Write data to the pipe.
    fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()>;
}

impl RichClient {
    pub fn new(client_id: u64, pipe_paths: Vec<String>) -> Self {
        Self {
            client_id,
            pipe_paths,
            #[cfg(target_os = "windows")]
            pipe: None,
            #[cfg(not(target_os = "windows"))]
            read_pipe: None,
            #[cfg(not(target_os = "windows"))]
            write_pipe: None,
            pid: std::process::id(),
            is_ready: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
            is_reconnecting: false,
            status: Status::Disconnected,
        }
    }

    /// Establishes a connection with Discord.
    pub fn connect(&mut self) -> crate::Result<()> {
        if self.pipe_paths.is_empty() {
            for pipe in get_dirs() {
                if self.try_connect(&pipe)? {
                    return Ok(());
                }
            }
        } else {
            let pipes = std::mem::take(&mut self.pipe_paths);

            for pipe in &pipes {
                if self.try_connect(pipe)? {
                    self.pipe_paths = pipes;
                    return Ok(());
                }
            }

            self.pipe_paths = pipes;
        }

        Err(DiscordError::PipeNotFound.into())
    }

    /// Sends a handshake packet to Discord.
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
        self.is_reconnecting = true;
        self.close();

        std::thread::sleep(Duration::from_millis(500));

        let mut client = Self::new(self.client_id, self.pipe_paths.clone());
        while self.is_reconnecting {
            if client.connect().is_ok() {
                if client.handshake().is_ok() {
                    *self = client;
                    let _ = self.start_read_thread(tx);

                    break;
                } else {
                    client.close();
                }
            };

            std::thread::sleep(Duration::from_millis(interval));
        }

        self.is_reconnecting = false;
    }
}

#[cfg(target_os = "windows")]
fn get_dirs() -> impl Iterator<Item = String> {
    (0..=10).map(|i| format!(r"\\.\pipe\discord-ipc-{}", i))
}

#[cfg(not(target_os = "windows"))]
fn get_dirs() -> impl Iterator<Item = String> {
    ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"]
        .iter()
        .filter_map(|&dir| std::env::var(dir).ok())
        .chain(["/tmp".to_string()])
        .flat_map(|base| {
            (0..10).map(move |i| format!("{}/discord-ipc-{}", base, i))
        })
}
