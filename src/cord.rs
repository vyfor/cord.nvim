use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::Duration,
};

use crate::{
    ipc::{
        discord::client::{Connection, RichClient},
        pipe::{platform::server::PipeServer, PipeServerImpl},
    },
    local_event,
    messages::{
        events::{
            event::{EventContext, OnEvent},
            local::ErrorEvent,
        },
        message::Message,
    },
    server_event,
    types::config::PluginConfig,
    util::logger::{LogLevel, Logger},
};

pub struct Cord {
    pub config: Config,
    pub plugin_config: Option<PluginConfig>,
    pub rich_client: Arc<RichClient>,
    pub pipe: PipeServer,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
    pub logger: Logger,
}

impl Cord {
    pub fn new(config: Config) -> crate::Result<Self> {
        let (tx, rx) = mpsc::channel::<Message>();
        let rich_client = Arc::new(RichClient::connect(config.client_id)?);
        let server = PipeServer::new(&config.pipe_name, tx.clone());
        let logger = Logger::new(tx.clone(), LogLevel::Off);

        Ok(Self {
            config,
            plugin_config: None,
            rich_client,
            pipe: server,
            tx,
            rx,
            logger,
        })
    }

    pub fn run(&mut self) -> crate::Result<()> {
        self.start_rpc()?;
        self.pipe.start()?;
        self.start_event_loop()?;

        Ok(())
    }

    fn start_event_loop(&mut self) -> crate::Result<()> {
        loop {
            if let Ok(msg) = self
                .rx
                .recv_timeout(Duration::from_millis(self.config.timeout))
            {
                msg.event.on_event(&mut EventContext {
                    cord: self,
                    client_id: msg.client_id,
                })?;
            } else if self.pipe.clients.read().unwrap().is_empty() {
                break;
            }
        }

        self.cleanup();
        Ok(())
    }

    fn start_rpc(&self) -> crate::Result<()> {
        self.rich_client.handshake()?;
        let rich_client = self.rich_client.clone();
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            if let Err(e) = rich_client.read() {
                tx.send(local_event!(0, Error, ErrorEvent::new(Box::new(e))))
                    .ok();
            } else {
                tx.send(server_event!(0, Ready)).ok();
            }
        });

        Ok(())
    }

    fn cleanup(&mut self) {
        if let Some(client) = Arc::get_mut(&mut self.rich_client) {
            client.close();
        }

        self.pipe.stop();
    }
}

pub struct Config {
    pub pipe_name: String,
    pub client_id: u64,
    pub timeout: u64,
}

impl Config {
    pub fn new(pipe_name: String, client_id: u64, timeout: u64) -> Self {
        Self {
            pipe_name,
            client_id,
            timeout,
        }
    }
}
