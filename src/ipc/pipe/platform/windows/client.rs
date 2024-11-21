use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::ipc::pipe::message::{ClientMessage, Event, LocalMessage, Message};
use crate::ipc::pipe::PipeClientImpl;

pub struct PipeClient {
    id: u32,
    pipe: Option<Arc<File>>,
    tx: Sender<Message>,
    thread_handle: Option<JoinHandle<()>>,
}

impl PipeClientImpl for PipeClient {
    type PipeType = File;

    fn new(id: u32, pipe: File, tx: Sender<Message>) -> Self {
        Self {
            id,
            pipe: Some(Arc::new(pipe)),
            tx,
            thread_handle: None,
        }
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.pipe.as_mut().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |pipe| pipe.write_all(data),
        )
    }

    fn start_read_thread(&mut self) -> io::Result<()> {
        if let Some(mut pipe) = self.pipe.take() {
            let tx = self.tx.clone();
            let id = self.id;

            let handle = std::thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    match pipe.read(&mut buf) {
                        Ok(n) if n == 0 => {
                            tx.send(Message::new(
                                id,
                                Event::Local(LocalMessage::ClientDisconnected),
                            ))
                            .ok();
                            break;
                        }
                        Ok(n) => {
                            if let Ok(message) =
                                ClientMessage::deserialize(&String::from_utf8_lossy(&buf[..n]))
                            {
                                tx.send(Message::new(id, Event::Client(message))).ok();
                            }
                        }
                        Err(e) => {
                            tx.send(Message::new(
                                id,
                                Event::Local(LocalMessage::Error(Box::new(e))),
                            ))
                            .ok();
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
            let _ = self.pipe.take();
        }
        if let Some(handle) = self.thread_handle.take() {
            drop(handle);
        }
    }
}
