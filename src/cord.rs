use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use crate::ipc::discord::client::Connection;
use crate::ipc::pipe::PipeServerImpl;
use crate::ipc::pipe::platform::server::PipeServer;
use crate::messages::events::event::{Event, EventContext, OnEvent};
use crate::messages::events::server::{LogEvent, ServerEvent};
use crate::messages::message::Message;
use crate::presence::manager::ActivityManager;
use crate::protocol::msgpack::Serialize;
use crate::session::SessionManager;
use crate::util::lockfile::ServerLock;
use crate::util::logger::{self, LOGGER, LogLevel, Logger};

pub const VERSION: &str = include_str!("../.github/server-version.txt");

/// Core application managing configuration, sessions, IPC with Discord, and logging.
///
/// # Fields
/// * `config`: Configuration settings.
/// * `session_manager`: Manages user sessions.
/// * `activity_manager`: Manages rich presence activity.
/// * `pipe`: Server-side communication pipe.
/// * `tx`, `rx`: Channels for message passing.
/// * `logger`: Logs application events.
/// * `_lock`: Ensures single instance operation.
pub struct Cord {
    pub config: Config,
    pub session_manager: Arc<SessionManager>,
    pub activity_manager: ActivityManager,
    pub pipe: PipeServer,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
    pub log_buffer: VecDeque<LogEvent>,
    _lock: ServerLock,
}

impl Cord {
    /// Initializes the Cord application.
    pub fn new(config: Config) -> crate::Result<Self> {
        let lock = ServerLock::new()?;

        let (tx, rx) = mpsc::channel::<Message>();
        let session_manager = Arc::new(SessionManager::default());
        let _ = logger::LOGGER.set(Logger::new(tx.clone(), LogLevel::Trace));

        let activity_manager = ActivityManager::new(config.client_id, vec![]);

        let server = PipeServer::new(
            &config.server_pipe,
            tx.clone(),
            Arc::clone(&session_manager),
        );

        Ok(Cord {
            config,
            session_manager,
            activity_manager,
            pipe: server,
            tx,
            rx,
            log_buffer: VecDeque::with_capacity(100),
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
                    } else if let Some(logger) = LOGGER.get()
                        && logger.would_log(LogLevel::Error)
                        && let Ok(data) =
                            LogEvent::new(e.to_string(), LogLevel::Error)
                                .to_msgpack()
                    {
                        while let Ok(ev) = self.rx.try_recv() {
                            match ev.event {
                                Event::Server(sev)
                                    if matches!(sev, ServerEvent::Log(_)) =>
                                {
                                    let _ = sev.on_event(&mut EventContext {
                                        cord: self,
                                        client_id: msg.client_id,
                                    });
                                }
                                _ => {}
                            }
                        }

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
        if let Ok(mut client) = self.activity_manager.client.write() {
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
