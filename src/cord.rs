use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
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
    types::Config,
};

pub struct Cord {
    pub config: Option<Config>,
    pub rich_client: Arc<RichClient>,
    pub pipe: PipeServer,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
}

impl Cord {
    pub fn new(pipe_name: &str, client_id: u64) -> crate::Result<Self> {
        let (tx, rx) = mpsc::channel::<Message>();
        let rich_client = Arc::new(RichClient::connect(client_id)?);
        let server = PipeServer::new(pipe_name, tx.clone());

        Ok(Self {
            config: None,
            rich_client,
            pipe: server,
            tx,
            rx,
        })
    }

    pub fn run(&mut self) -> crate::Result<()> {
        self.start_rpc()?;
        self.pipe.start()?;
        self.start_event_loop()?;

        Ok(())
    }

    fn start_event_loop(&mut self) -> crate::Result<()> {
        while let Ok(msg) = self.rx.recv() {
            msg.event.on_event(&EventContext {
                cord: self,
                client_id: msg.client_id,
            })?;
        }

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
}
