use std::os::windows::fs::OpenOptionsExt;
use std::{fs::OpenOptions, io};

use crate::ipc::discord::client::{Connection, RichClient};

impl Connection for RichClient {
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
                        last_activity: None,
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
