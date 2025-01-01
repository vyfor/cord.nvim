use std::ffi::OsStr;
use std::fs::File;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{io, ptr};

use crate::ipc::bindings::{
    CreateEventW, CreateFileW, GetLastError, GetOverlappedResult, Overlapped,
    ReadFile, WriteFile, ERROR_IO_PENDING, FILE_FLAG_OVERLAPPED, GENERIC_READ,
    GENERIC_WRITE, INVALID_HANDLE_VALUE, OPEN_EXISTING,
};
use crate::ipc::discord::client::{Connection, RichClient};
use crate::ipc::discord::error::DiscordError;
use crate::ipc::discord::opcodes::Opcode;
use crate::ipc::discord::utils;
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::{local_event, server_event};

impl Connection for RichClient {
    /// Pipe path can be under the directory `\\\\.\\pipe\\discord-ipc-{i}` where `i` is a number from 0 to 9.
    fn connect(client_id: u64) -> crate::Result<Self> {
        for i in 0..10 {
            let pipe_name = format!("\\\\.\\pipe\\discord-ipc-{i}");
            let wide_name: Vec<u16> = OsStr::new(&pipe_name)
                .encode_wide()
                .chain(Some(0))
                .collect();

            unsafe {
                let handle = CreateFileW(
                    wide_name.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_OVERLAPPED,
                    0 as _,
                );

                if handle == INVALID_HANDLE_VALUE {
                    let error = io::Error::last_os_error();
                    match error.kind() {
                        io::ErrorKind::NotFound => continue,
                        _ => return Err(DiscordError::Io(error).into()),
                    }
                }

                let pipe = File::from_raw_handle(handle);
                let client = RichClient {
                    client_id,
                    pipe: Some(Arc::new(pipe)),
                    pid: std::process::id(),
                    is_ready: Arc::new(false.into()),
                    thread_handle: None,
                    is_reconnecting: Arc::new(false.into()),
                };

                return Ok(client);
            }
        }

        Err(DiscordError::PipeNotFound.into())
    }

    fn close(&mut self) {
        self.pipe = None;
        if let Some(handle) = self.thread_handle.take() {
            handle.join().ok();
        }
    }

    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()> {
        if let Some(pipe) = self.pipe.as_ref() {
            let pipe = pipe.clone();
            let client_id = self.client_id;
            let is_ready = self.is_ready.clone();

            let handle = std::thread::spawn(move || {
                let mut buf = [0u8; 8192];
                let handle = pipe.as_raw_handle();

                loop {
                    unsafe {
                        let h_event = CreateEventW(
                            ptr::null_mut(),
                            1,
                            0,
                            ptr::null_mut(),
                        );

                        let mut overlapped = Overlapped {
                            internal: 0,
                            internal_high: 0,
                            offset: 0,
                            offset_high: 0,
                            h_event,
                        };

                        let mut bytes_read = 0;
                        let read_result = ReadFile(
                            handle,
                            buf.as_mut_ptr(),
                            buf.len() as u32,
                            &mut bytes_read,
                            &mut overlapped,
                        );

                        if read_result == 0 {
                            let error = GetLastError();
                            if error != ERROR_IO_PENDING {
                                tx.send(local_event!(
                                    0,
                                    Error,
                                    ErrorEvent::new(Box::new(
                                        DiscordError::ConnectionClosed
                                    ))
                                ))
                                .ok();
                                break;
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
                            tx.send(local_event!(
                                0,
                                Error,
                                ErrorEvent::new(Box::new(
                                    DiscordError::ConnectionClosed
                                ))
                            ))
                            .ok();
                            break;
                        }

                        if bytes_transferred >= 8 {
                            if let Some((opcode, size)) = utils::decode(
                                &buf[..bytes_transferred as usize],
                            ) {
                                if size > 0 && bytes_transferred >= (8 + size) {
                                    let data = &buf[8..8 + size as usize];
                                    let data_str =
                                        String::from_utf8_lossy(data);

                                    match Opcode::from(opcode) {
                                        Opcode::Frame => {
                                            if data_str
                                                .contains("Invalid Client ID")
                                            {
                                                tx.send(local_event!(
                                                    0,
                                                    Error,
                                                    ErrorEvent::new(Box::new(
                                                        DiscordError::InvalidClientId(
                                                            client_id.to_string()
                                                        )
                                                    ))
                                                ))
                                                .ok();
                                                break;
                                            }
                                            if !is_ready
                                                .swap(true, Ordering::SeqCst)
                                            {
                                                tx.send(server_event!(
                                                    0, Ready
                                                ))
                                                .ok();
                                            }
                                        }
                                        Opcode::Close => {
                                            tx.send(local_event!(
                                                0,
                                                Error,
                                                ErrorEvent::new(Box::new(
                                                    DiscordError::ConnectionClosed
                                                ))
                                            ))
                                            .ok();
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            });

            self.thread_handle = Some(handle);
            Ok(())
        } else {
            Err(DiscordError::PipeNotFound.into())
        }
    }

    fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()> {
        self.pipe.as_ref().map_or(Ok(()), |pipe| {
            let payload = match data {
                Some(packet) => {
                    let mut payload =
                        utils::encode(opcode, packet.len() as u32);
                    payload.extend_from_slice(packet);
                    payload
                }
                None => utils::encode(opcode, 0),
            };

            unsafe {
                let handle = pipe.as_raw_handle();
                let h_event =
                    CreateEventW(ptr::null_mut(), 1, 0, ptr::null_mut());

                let mut overlapped = Overlapped {
                    internal: 0,
                    internal_high: 0,
                    offset: 0,
                    offset_high: 0,
                    h_event,
                };

                let mut bytes_written = 0;
                let write_result = WriteFile(
                    handle,
                    payload.as_ptr(),
                    payload.len() as u32,
                    &mut bytes_written,
                    &mut overlapped,
                );

                if write_result == 0 {
                    let error = GetLastError();
                    if error != ERROR_IO_PENDING {
                        return Err(DiscordError::ConnectionClosed.into());
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
                    return Err(DiscordError::ConnectionClosed.into());
                }

                Ok(())
            }
        })
    }
}
