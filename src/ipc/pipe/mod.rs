pub mod message;
pub mod platform;
pub mod types;

use std::{io, sync::mpsc::Sender};

use message::Message;

pub trait PipeServerImpl {
    fn new(pipe_name: &str, tx: Sender<Message>) -> Self
    where
        Self: Sized;
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self);
    fn broadcast(&self, data: &[u8]) -> io::Result<()>;
    fn write_to(&self, client_id: u32, data: &[u8]) -> io::Result<()>;
    fn disconnect(&mut self, client_id: u32) -> io::Result<()>;
}

pub trait PipeClientImpl {
    fn new(id: u32, pipe: Self::PipeType, tx: Sender<Message>) -> Self
    where
        Self: Sized;
    fn write(&mut self, data: &[u8]) -> io::Result<()>;
    fn start_read_thread(&mut self) -> io::Result<()>;

    type PipeType;
}
