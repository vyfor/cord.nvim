use std::fs::OpenOptions;
use std::io;
use std::os::windows::fs::OpenOptionsExt;

use crate::ipc::discord::client::{Connection, RichClient};

impl Connection for RichClient {
    /// Pipe path can be under the directory `\\\\.\\pipe\\discord-ipc-{i}` where `i` is a number from 0 to 9.
    fn connect(client_id: u64) -> crate::Result<Self> {
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
                        pid: std::process::id(),
                        is_ready: false.into(),
                    })
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => continue,
                    _ => return Err(e.into()),
                },
            }
        }

        Err("Pipe not found".into())
    }

    fn close(&mut self) {
        self.pipe = None;
    }
}
