use std::ffi::OsStr;
use std::fs::File;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::{io, ptr};

use crate::ipc::bindings::{
    CreateEventW, CreateFileW, ERROR_IO_PENDING, FILE_FLAG_OVERLAPPED,
    GENERIC_READ, GENERIC_WRITE, GetLastError, GetOverlappedResult,
    INVALID_HANDLE_VALUE, OPEN_EXISTING, Overlapped, ReadFile, WriteFile,
};
use crate::ipc::discord::client::{Connection, RichClient};
use crate::ipc::discord::error::DiscordError;
use crate::ipc::discord::opcodes::Opcode;
use crate::ipc::discord::utils;
use crate::messages::events::local::ErrorEvent;
use crate::messages::events::server::StatusUpdateEvent;
use crate::messages::message::Message;
use crate::{debug, local_event, server_event, trace};

impl Connection for RichClient {
    /// Pipe can be under the path `\\\\.\\pipe\\discord-ipc-{i}` where `i` is a number from 0 to 9.
    fn try_connect(&mut self, pipe_name: &str) -> crate::Result<bool> {
        trace!("Attempting to connect to Windows named pipe: {}", pipe_name);
        let wide_name: Vec<u16> =
            OsStr::new(pipe_name).encode_wide().chain(Some(0)).collect();

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
                return match error.kind() {
                    io::ErrorKind::NotFound => {
                        trace!("Windows named pipe not found: {}", pipe_name);
                        Ok(false)
                    }
                    _ => {
                        debug!(
                            "Failed to connect to Windows named pipe: {}",
                            error
                        );
                        Ok(false)
                    }
                };
            }

            let pipe = File::from_raw_handle(handle);
            self.pipe = Some(pipe.into());
            debug!("Successfully connected to Windows named pipe");
            Ok(true)
        }
    }

    fn close(&mut self) {
        debug!("Closing Discord Windows named pipe connection");
        self.pipe = None;
        let _ = self.thread_handle.take();
    }

    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()> {
        if let Some(pipe) = self.pipe.as_ref() {
            let pipe = pipe.clone();
            let client_id = self.client_id;
            let is_ready = self.is_ready.clone();

            debug!("Starting Discord IPC read thread (Windows)");
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
                                debug!(
                                    "Discord IPC read failed with error code: {}",
                                    error
                                );
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
                            debug!("Discord IPC GetOverlappedResult failed");
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

                        trace!(
                            "Received {} bytes from Discord IPC",
                            bytes_transferred
                        );
                        if bytes_transferred >= 8
                            && let Some((opcode, size)) = utils::decode(
                                &buf[..bytes_transferred as usize],
                            )
                            && size > 0
                            && bytes_transferred >= (8 + size)
                        {
                            let data = &buf[8..8 + size as usize];
                            let data_str = String::from_utf8_lossy(data);
                            trace!(
                                "Received message from Discord: opcode={}, data={}",
                                opcode, data_str
                            );

                            match Opcode::from(opcode) {
                                Opcode::Frame => {
                                    if data_str.contains("Invalid Client ID") {
                                        debug!(
                                            "Discord reported invalid client ID"
                                        );
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
                                    if !is_ready.swap(true, Ordering::SeqCst) {
                                        debug!(
                                            "Discord IPC connection is now ready"
                                        );
                                        tx.send(server_event!(
                                            0,
                                            StatusUpdate,
                                            StatusUpdateEvent::ready()
                                        ))
                                        .ok();
                                    }
                                }
                                Opcode::Close => {
                                    debug!(
                                        "Received close opcode from Discord"
                                    );
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
                                _ => {
                                    trace!(
                                        "Received unhandled opcode: {:?}",
                                        Opcode::from(opcode)
                                    );
                                }
                            }
                        }
                    }
                }
            });

            self.thread_handle = Some(handle);
            Ok(())
        } else {
            debug!("Cannot start read thread: no pipe available");
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

            trace!(
                "Writing {} bytes to Discord IPC (opcode={})",
                payload.len(),
                opcode
            );
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
                        debug!(
                            "Discord IPC write failed with error code: {}",
                            error
                        );
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
                    debug!("Discord IPC write GetOverlappedResult failed");
                    return Err(DiscordError::ConnectionClosed.into());
                }

                trace!(
                    "Successfully wrote {} bytes to Discord IPC",
                    bytes_transferred
                );
                Ok(())
            }
        })
    }
}
