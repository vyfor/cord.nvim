use std::{env::args, sync::mpsc};

use ipc::{
    discord::client::{Connection, RichClient},
    pipe::{platform::server::PipeServer, PipeServerImpl},
};
use messages::{
    events::{event::Event, local::LocalEvent},
    message::Message,
};

mod ipc;
mod json;
mod mappings;
mod messages;
mod presence;
mod types;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = args().nth(1).ok_or("Missing client ID")?.parse::<u64>()?;
    let (tx, rx) = mpsc::channel::<Message>();
    let mut _rich_client = RichClient::connect(client_id)?;
    let mut pipe = PipeServer::new("cord-ipc", tx);
    pipe.start()?;

    while let Ok(message) = rx.recv() {
        match message.event {
            Event::Client(_client_message) => {}
            Event::Local(server_message) => match server_message {
                LocalEvent::ClientDisconnected(_) => {
                    println!("Client {} disconnected", message.client_id);
                    break;
                }
                LocalEvent::Error(event) => {
                    println!("Error: {}", event.error);
                    break;
                }
            },
        }
    }

    Ok(())
}
