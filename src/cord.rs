use std::{
    io,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
};

use crate::{
    ipc::{
        discord::client::{Connection, RichClient},
        pipe::{platform::server::PipeServer, PipeServerImpl},
    },
    local_event,
    messages::{events::local::ErrorEvent, handler::MessageHandler, message::Message},
    server_event,
    types::Config,
};

pub struct Cord {
    pub config: Option<Config>,
    pub message_handler: MessageHandler,
    pub rich_client: Arc<RichClient>,
    pub server: PipeServer,
    pub tx: Sender<Message>,
}

impl Cord {
    pub fn new(pipe_name: &str, client_id: u64) -> io::Result<Self> {
        let (tx, rx) = mpsc::channel::<Message>();
        let message_handler = MessageHandler::new(rx);
        let rich_client = Arc::new(RichClient::connect(client_id)?);
        let server = PipeServer::new(pipe_name, tx.clone());

        Ok(Self {
            config: None,
            message_handler,
            rich_client,
            server,
            tx,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.start_rpc()?;
        self.server.start()?;
        self.message_handler.run(&self.server);

        Ok(())
    }

    fn start_rpc(&self) -> io::Result<()> {
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
}
