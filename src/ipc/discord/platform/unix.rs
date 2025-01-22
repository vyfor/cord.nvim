use std::env::var;
use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

use crate::ipc::discord::client::{Connection, RichClient};
use crate::ipc::discord::error::DiscordError;
use crate::ipc::discord::opcodes::Opcode;
use crate::ipc::discord::utils;
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::{local_event, server_event};

impl Connection for RichClient {
    /// Pipe path can be in any of the following directories:
    /// * `XDG_RUNTIME_DIR`
    /// * `TMPDIR`
    /// * `TMP`
    /// * `TEMP`
    /// * `/tmp`
    ///
    /// Followed by:
    /// * `/app/com.discordapp.Discord` - for flatpak
    /// * `/snap.discord` - for snap
    ///
    /// Followed by:
    /// * `/discord-ipc-{i}` - where `i` is a number from 0 to 9
    fn connect(&mut self) -> crate::Result<()> {
        let dirs = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"]
            .iter()
            .filter_map(|&dir| var(dir).ok())
            .chain(["/tmp".to_string()])
            .flat_map(|base| {
                [
                    base.to_string(),
                    format!("{}/app/com.discordapp.Discord", base),
                    format!("{}/snap.discord", base),
                ]
            });

        for dir in dirs {
            for i in 0..10 {
                match UnixStream::connect(format!("{dir}/discord-ipc-{i}")) {
                    Ok(pipe) => {
                        let read_pipe =
                            pipe.try_clone().map_err(DiscordError::Io)?;
                        self.read_pipe = Some(read_pipe);
                        self.write_pipe = Some(pipe);
                        return Ok(());
                    }
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => continue,
                        _ => return Err(DiscordError::Io(e).into()),
                    },
                }
            }
        }

        Err(DiscordError::PipeNotFound.into())
    }

    fn close(&mut self) {
        if let Some(pipe) = self.read_pipe.take() {
            let _ = pipe.shutdown(Shutdown::Both);
        }
        if let Some(pipe) = self.write_pipe.take() {
            let _ = pipe.shutdown(Shutdown::Both);
        }
        let _ = self.thread_handle.take();
    }

    fn start_read_thread(&mut self, tx: Sender<Message>) -> crate::Result<()> {
        if let Some(mut read_pipe) = self.read_pipe.take() {
            let client_id = self.client_id;
            let is_ready = self.is_ready.clone();

            let handle = std::thread::spawn(move || {
                let mut buf = [0u8; 8192];
                loop {
                    match read_pipe.read(&mut buf) {
                        Ok(0) => {
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
                        Ok(bytes_transferred) => {
                            if bytes_transferred >= 8 {
                                if let Some((opcode, size)) =
                                    utils::decode(&buf[..bytes_transferred])
                                {
                                    if size > 0
                                        && bytes_transferred
                                            >= 8 + size as usize
                                    {
                                        let data = &buf[8..8 + size as usize];
                                        let data_str =
                                            String::from_utf8_lossy(data);

                                        match Opcode::from(opcode) {
                                            Opcode::Frame => {
                                                if data_str.contains(
                                                    "Invalid Client ID",
                                                ) {
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
                                                if !is_ready.swap(
                                                    true,
                                                    Ordering::SeqCst,
                                                ) {
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
                        Err(_) => {
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
                }
            });

            self.thread_handle = Some(handle);
            Ok(())
        } else {
            Err(DiscordError::PipeNotFound.into())
        }
    }

    fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()> {
        self.write_pipe.as_ref().map_or(Ok(()), |mut pipe| {
            let payload = match data {
                Some(packet) => {
                    let mut payload =
                        utils::encode(opcode, packet.len() as u32);
                    payload.extend_from_slice(packet);
                    payload
                }
                None => utils::encode(opcode, 0),
            };

            match pipe.write_all(&payload) {
                Ok(_) => Ok(()),
                Err(_) => Err(DiscordError::ConnectionClosed.into()),
            }
        })
    }
}
