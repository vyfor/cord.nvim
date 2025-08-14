use std::fs::File;
use std::io::{self};
use std::os::windows::io::AsRawHandle;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use crate::ipc::bindings::{
    ERROR_IO_PENDING, GetOverlappedResult, Overlapped, ReadFile, WriteFile,
};
use crate::ipc::pipe::{PipeClientImpl, report_error};
use crate::messages::events::client::ClientEvent;
use crate::messages::events::event::Event;
use crate::messages::events::server::LogEvent;
use crate::messages::message::Message;
use crate::util::logger::LogLevel;
use crate::{client_event, server_event};

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
                let data_len = data.len();
                let mut framed_data = Vec::with_capacity(4 + data_len);
                framed_data.extend_from_slice(&(data_len as u32).to_be_bytes());
                framed_data.extend_from_slice(data);

                let mut overlapped = Overlapped::default();
                let mut bytes_written = 0;

                let write_result = WriteFile(
                    handle,
                    framed_data.as_ptr(),
                    framed_data.len() as u32,
                    &mut bytes_written,
                    &mut overlapped,
                );

                if write_result == 0 {
                    let error = io::Error::last_os_error();
                    if error.raw_os_error() != Some(ERROR_IO_PENDING as i32) {
                        return Err(error);
                    }
                }

                let mut bytes_transferred = 0;
                if GetOverlappedResult(
                    handle,
                    &mut overlapped,
                    &mut bytes_transferred,
                    1,
                ) == 0
                {
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
                            if error.raw_os_error()
                                != Some(ERROR_IO_PENDING as i32)
                            {
                                report_error(id, &tx, error);
                                break;
                            }
                        }

                        if GetOverlappedResult(
                            pipe.as_raw_handle(),
                            &mut overlapped,
                            &mut bytes_read,
                            1,
                        ) == 0
                        {
                            report_error(id, &tx, io::Error::last_os_error());
                            break;
                        }

                        if bytes_read == 0 {
                            tx.send(client_event!(id, Disconnect)).ok();
                            break;
                        }

                        match ClientEvent::deserialize(
                            &buf[..bytes_read as usize],
                        ) {
                            Ok(message) => {
                                tx.send(Message::new(
                                    id,
                                    Event::Client(message),
                                ))
                                .ok();
                            }
                            Err(e) => {
                                tx.send(server_event!(
                                    id,
                                    Log,
                                    LogEvent::new(
                                        e.to_string(),
                                        LogLevel::Error
                                    )
                                ))
                                .ok();
                            }
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
