use std::{env::args, sync::mpsc};

use ipc::{
    discord::client::{Connection, RichClient},
    pipe::{
        message::{Message, ServerMessage},
        platform::server::PipeServer,
        PipeServerImpl,
    },
};

mod activity;
mod ipc;
mod json;
mod mappings;
mod types;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = args().nth(1).ok_or("Missing client ID")?.parse::<u64>()?;
    let (tx, rx) = mpsc::channel::<Message>();
    let mut _rich_client = RichClient::connect(client_id)?;
    let mut pipe = PipeServer::new("cord-ipc", tx);
    pipe.start()?;

    while let Ok(message) = rx.recv() {
        match message {
            Message::Client(_client_message) => {}
            Message::Server(server_message) => match server_message {
                ServerMessage::ClientDisconnected(client_id) => {
                    println!("Client {} disconnected", client_id);
                    break;
                }
                ServerMessage::Error(e) => {
                    println!("Error: {}", e);
                    break;
                }
            },
        }
    }

    Ok(())
}
