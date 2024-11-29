pub mod platform;

use std::sync::Arc;
use std::{io, sync::mpsc::Sender};

use crate::client_event;
use crate::{
    local_event,
    messages::{events::local::ErrorEvent, message::Message},
    session::SessionManager,
};

pub trait PipeServerImpl {
    fn new(pipe_name: &str, tx: Sender<Message>, session_manager: Arc<SessionManager>) -> Self
    where
        Self: Sized;
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self);
    fn broadcast(&self, data: &[u8]) -> io::Result<()>;
    fn write_to(&self, client_id: u32, data: &[u8]) -> io::Result<()>;
    fn disconnect(&self, client_id: u32) -> io::Result<()>;
}

pub trait PipeClientImpl {
    fn new(id: u32, pipe: Self::PipeType, tx: Sender<Message>) -> Self
    where
        Self: Sized;
    fn write(&mut self, data: &[u8]) -> io::Result<()>;
    fn start_read_thread(&mut self) -> io::Result<()>;

    type PipeType;
}

fn report_error(id: u32, tx: &Sender<Message>, error: io::Error) {
    if error.kind() == io::ErrorKind::BrokenPipe {
        tx.send(client_event!(id, Disconnect)).ok();
        return;
    }

    tx.send(local_event!(0, Error, ErrorEvent::new(Box::new(error))))
        .ok();
}
