use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::error::CordErrorKind;
use crate::ipc::discord::client::{Connection, RichClient};
use crate::ipc::pipe::platform::server::PipeServer;
use crate::ipc::pipe::PipeServerImpl;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::protocol::msgpack::MsgPack;
use crate::session::SessionManager;
use crate::util::lockfile::ServerLock;
use crate::util::logger::{LogLevel, Logger};

/// Core application managing configuration, sessions, IPC with Discord, and logging.
///
/// # Fields
/// * `config`: Configuration settings.
/// * `session_manager`: Manages user sessions.
/// * `rich_client`: Handles communication with Discord.
/// * `pipe`: Server-side communication pipe.
/// * `tx`, `rx`: Channels for message passing.
/// * `logger`: Logs application events.
/// * `_lock`: Ensures single instance operation.
pub struct Cord {
    pub config: Config,
    pub session_manager: Arc<SessionManager>,
    pub rich_client: Arc<RwLock<RichClient>>,
    pub pipe: PipeServer,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
    pub logger: Arc<Logger>,
    _lock: ServerLock,
}

impl Cord {
    /// Initializes the Cord application.
    pub fn new(config: Config) -> crate::Result<Self> {
        let lock = ServerLock::new()?;

        let (tx, rx) = mpsc::channel::<Message>();
        let session_manager = Arc::new(SessionManager::default());
        let rich_client = match RichClient::connect(config.client_id) {
            Ok(client) => Arc::new(RwLock::new(client)),
            Err(_) => {
                return Err(crate::error::CordError::new(
                    CordErrorKind::Io,
                    "Failed to connect to Discord",
                ));
            }
        };
        let server = PipeServer::new(
            &config.pipe_name,
            tx.clone(),
            Arc::clone(&session_manager),
        );
        let logger = Arc::new(Logger::new(tx.clone(), LogLevel::Off));

        Ok(Cord {
            config,
            session_manager,
            rich_client,
            pipe: server,
            tx,
            rx,
            logger,
            _lock: lock,
        })
    }

    /// Runs the application.
    pub fn run(&mut self) -> crate::Result<()> {
        self.start_rpc()?;
        self.pipe.start()?;
        self.start_event_loop()?;

        Ok(())
    }

    /// Starts the event loop.
    pub fn start_event_loop(&mut self) -> crate::Result<()> {
        loop {
            if let Ok(msg) = self
                .rx
                .recv_timeout(Duration::from_millis(self.config.timeout))
            {
                if let Err(e) = msg.event.on_event(&mut EventContext {
                    cord: self,
                    client_id: msg.client_id,
                }) {
                    if self.session_manager.sessions.read().unwrap().is_empty()
                    {
                        return Err(e);
                    } else if let Ok(data) = MsgPack::serialize(&LogEvent::new(
                        e.to_string(),
                        LogLevel::Error,
                    )) {
                        self.pipe.broadcast(&data)?;
                        return Ok(());
                    }

                    return Err(e);
                }
            } else if self.session_manager.sessions.read().unwrap().is_empty() {
                break;
            }
        }

        self.cleanup();
        Ok(())
    }

    /// Starts RPC with Discord.
    pub fn start_rpc(&mut self) -> crate::Result<()> {
        let mut rich_client = self
            .rich_client
            .write()
            .expect("Failed to lock rich client");
        rich_client.handshake()?;
        rich_client.start_read_thread(self.tx.clone())?;

        Ok(())
    }

    /// Cleans up before shutdown.
    pub fn cleanup(&mut self) {
        if let Some(client) = Arc::get_mut(&mut self.rich_client) {
            if let Ok(client) = client.get_mut() {
                client.close();
            }
        }

        self.pipe.stop();
    }

    /// Shuts down the application.
    #[inline(always)]
    pub fn shutdown(&mut self) {
        self.cleanup();
        std::process::exit(0);
    }
}

/// Manages application settings required for initialization.
pub struct Config {
    pub pipe_name: String,
    pub client_id: u64,
    pub timeout: u64,
    pub reconnect_interval: u64,
}

impl Config {
    /// Creates a new configuration.
    pub fn new(
        pipe_name: String,
        client_id: u64,
        timeout: u64,
        reconnect_interval: u64,
    ) -> Self {
        Self {
            pipe_name,
            client_id,
            timeout,
            reconnect_interval,
        }
    }
}
