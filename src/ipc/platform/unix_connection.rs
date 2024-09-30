use std::env::var;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

use crate::ipc::client::{Connection, RichClient};
use crate::ipc::utils;
use crate::rpc::packet::Packet;

impl Connection for RichClient {
    fn connect(client_id: u64) -> Result<Self, Box<dyn std::error::Error>> {
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
                        return Ok(RichClient {
                            client_id: client_id,
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
        }

        Err("Pipe not found".into())
    }

    fn write(&mut self, opcode: u32, data: Option<&[u8]>) -> io::Result<()> {
        if let Some(pipe) = &mut self.pipe {
            let payload = match data {
                Some(packet) => {
                    let mut payload =
                        utils::encode(opcode, packet.len() as u32);
                    payload.extend_from_slice(packet);
                    payload
                }
                None => utils::encode(opcode, 0),
            };
            pipe.write_all(&payload)?;
        }

        Ok(())
    }

    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.pipe
            .as_mut()
            .map_or(Err("Pipe not found".into()), |pipe| {
                let mut header = [0; 8];
                pipe.read_exact(&mut header)?;
                let size = utils::decode(&header) as usize;
                let mut buffer = vec![0u8; size];
                pipe.read_exact(&mut buffer)?;
                Ok(buffer)
            })
    }

    fn close(&mut self) {
        self.write(
            2,
            Some(
                format!("{{'v': 1, 'client_id': {}}}", self.client_id)
                    .as_bytes(),
            ),
        )
        .unwrap();
        if let Some(pipe) = self.pipe.take() {
            let _ = pipe.shutdown(std::net::Shutdown::Both);
        }
    }

    fn handshake(&mut self) -> io::Result<()> {
        self.write(
            0,
            Some(
                (format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id))
                    .as_bytes(),
            ),
        )
    }

    fn update(
        &mut self,
        packet: &crate::rpc::packet::Packet,
    ) -> io::Result<()> {
        if packet.activity != self.last_activity {
            return self.write(1, Some(packet.to_json().unwrap().as_bytes()));
        }

        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.write(
            1,
            Some(
                Packet {
                    pid: std::process::id(),
                    activity: None,
                }
                .to_json()
                .unwrap()
                .as_bytes(),
            ),
        )
    }
}
