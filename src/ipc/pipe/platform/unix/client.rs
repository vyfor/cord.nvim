use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use crate::ipc::pipe::{report_error, PipeClientImpl};
use crate::local_event;
use crate::messages::events::client::ClientEvent;
use crate::messages::events::event::Event;
use crate::messages::message::Message;

pub struct PipeClient {
    id: u32,
    read_pipe: Option<UnixStream>,
    write_pipe: Option<UnixStream>,
    tx: Sender<Message>,
    thread_handle: Option<JoinHandle<()>>,
}

impl PipeClientImpl for PipeClient {
    type PipeType = UnixStream;

    fn new(id: u32, pipe: UnixStream, tx: Sender<Message>) -> Self {
        let read_pipe = pipe.try_clone().unwrap();
        Self {
            id,
            read_pipe: Some(read_pipe),
            write_pipe: Some(pipe),
            tx,
            thread_handle: None,
        }
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_pipe.as_mut().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |pipe| pipe.write_all(data),
        )
    }

    fn start_read_thread(&mut self) -> io::Result<()> {
        if let Some(mut read_pipe) = self.read_pipe.take() {
            let tx = self.tx.clone();
            let id = self.id;

            let handle = std::thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    match read_pipe.read(&mut buf) {
                        Ok(0) => {
                            tx.send(local_event!(id, ClientDisconnected)).ok();
                            break;
                        }
                        Ok(n) => match ClientEvent::deserialize(&buf[..n]) {
                            Ok(message) => {
                                tx.send(Message::new(id, Event::Client(message))).ok();
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize message: {:?}", e);
                            }
                        },
                        Err(e) => {
                            report_error(&tx, e);
                            break;
                        }
                    }
                }
            });
            self.thread_handle = Some(handle);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found"))
        }
    }
}

impl Drop for PipeClient {
    fn drop(&mut self) {
        {
            let _ = self.read_pipe.take();
            let _ = self.write_pipe.take();
        }
        if let Some(handle) = self.thread_handle.take() {
            drop(handle);
        }
    }
}
