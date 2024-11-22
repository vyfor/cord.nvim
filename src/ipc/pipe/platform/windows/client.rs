use std::fs::File;
use std::io::{self};
use std::os::windows::io::AsRawHandle;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::JoinHandle;

use super::{
    Overlapped, ReadFile, WaitForSingleObject, WriteFile, ERROR_IO_PENDING, INFINITE, WAIT_OBJECT_0,
};
use crate::ipc::pipe::PipeClientImpl;
use crate::local_event;
use crate::messages::events::client::ClientEvent;
use crate::messages::events::event::Event;
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;

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
        if let Some(pipe) = &self.pipe {
            let handle = pipe.as_raw_handle();
            unsafe {
                let mut overlapped = Overlapped::default();

                let mut bytes_written = 0;
                let write_result = WriteFile(
                    handle,
                    data.as_ptr(),
                    data.len() as u32,
                    &mut bytes_written,
                    &mut overlapped,
                );

                if write_result == 0 {
                    let error = io::Error::last_os_error();
                    if error.raw_os_error() != Some(ERROR_IO_PENDING as i32) {
                        return Err(error);
                    }
                }

                if WaitForSingleObject(overlapped.h_event, INFINITE) != WAIT_OBJECT_0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found"))
        }
    }

    fn start_read_thread(&mut self) -> io::Result<()> {
        if let Some(pipe) = self.pipe.as_ref() {
            let pipe = pipe.clone();
            let tx = self.tx.clone();
            let id = self.id;

            let handle = std::thread::spawn(move || {
                let mut buf = [0u8; 4096];
                let handle = pipe.as_raw_handle();

                loop {
                    unsafe {
                        let mut overlapped = Overlapped::default();
                        let mut bytes_read = 0;
                        let read_result = ReadFile(
                            handle,
                            buf.as_mut_ptr(),
                            buf.len() as u32,
                            &mut bytes_read,
                            &mut overlapped,
                        );

                        if read_result == 0 {
                            let error = io::Error::last_os_error();
                            if error.raw_os_error() != Some(ERROR_IO_PENDING as i32) {
                                tx.send(local_event!(id, Error, ErrorEvent::new(Box::new(error))))
                                    .ok();
                                break;
                            }
                        }

                        if WaitForSingleObject(overlapped.h_event, INFINITE) != WAIT_OBJECT_0 {
                            tx.send(local_event!(
                                id,
                                Error,
                                ErrorEvent::new(Box::new(io::Error::last_os_error()))
                            ))
                            .ok();
                            break;
                        }

                        if bytes_read == 0 {
                            tx.send(local_event!(id, ClientDisconnected)).ok();
                            break;
                        }

                        if let Ok(message) = ClientEvent::deserialize(&String::from_utf8_lossy(
                            &buf[..bytes_read as usize],
                        )) {
                            tx.send(Message::new(id, Event::Client(message))).ok();
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
