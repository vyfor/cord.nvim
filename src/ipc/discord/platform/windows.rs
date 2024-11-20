use std::fs::OpenOptions;
use std::io;
use std::os::windows::fs::OpenOptionsExt;

use crate::ipc::discord::client::{Connection, RichClient};

impl Connection for RichClient {
    fn connect(client_id: u64) -> io::Result<Self> {
        for i in 0..10 {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .access_mode(0x3)
                .open(format!("\\\\.\\pipe\\discord-ipc-{i}"))
            {
                Ok(pipe) => {
                    return Ok(RichClient {
                        client_id,
                        pipe: Some(pipe),
                        last_activity: None,
                    })
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => continue,
                    _ => return Err(e.into()),
                },
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found"))
    }

    fn close(&mut self) {
        self.pipe = None;
    }
}
