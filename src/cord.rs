use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::ipc::discord::client::{Connection, RichClient};
use crate::ipc::pipe::PipeServerImpl;
use crate::ipc::pipe::platform::server::PipeServer;
use crate::messages::events::event::{EventContext, OnEvent};
use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::protocol::msgpack::MsgPack;
use crate::session::SessionManager;
use crate::util::lockfile::ServerLock;
use crate::util::logger::{LogLevel, Logger};

pub const VERSION: &str = include_str!("../.github/server-version.txt");

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
        let logger = Arc::new(Logger::new(tx.clone(), LogLevel::Off));

        let rich_client =
            Arc::new(RwLock::new(RichClient::new(config.client_id, vec![])));

        let server = PipeServer::new(
            &config.server_pipe,
            tx.clone(),
            Arc::clone(&session_manager),
        );

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

    /// Cleans up before shutdown.
    pub fn cleanup(&mut self) {
        if let Some(client) = Arc::get_mut(&mut self.rich_client)
            && let Ok(client) = client.get_mut()
        {
            client.close();
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
    pub server_pipe: String,
    pub client_id: u64,
    pub timeout: u64,
    pub reconnect_interval: u64,
    pub initial_reconnect: bool,
    pub shared_timestamps: bool,
}

impl Config {
    /// Creates a new configuration.
    pub fn new(
        server_pipe: String,
        client_id: u64,
        timeout: u64,
        reconnect_interval: u64,
        initial_reconnect: bool,
        shared_timestamps: bool,
    ) -> Self {
        Self {
            server_pipe,
            client_id,
            timeout,
            reconnect_interval,
            initial_reconnect,
            shared_timestamps,
        }
    }
}
