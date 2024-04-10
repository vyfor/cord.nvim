#![cfg(target_os = "windows")]

use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::os::windows::fs::OpenOptionsExt;

use crate::ipc::client::{Connection, RichClient};
use crate::ipc::utils;

impl Connection for RichClient {
    fn connect(client_id: u64) -> Result<Self, Box<dyn std::error::Error>> {
        for i in 0..10 {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .access_mode(0x3)
                .open(format!("\\\\.\\pipe\\discord-ipc-{}", i))
            {
                Ok(pipe) => {
                    return Ok(RichClient {
                        client_id: client_id,
                        pipe: pipe,
                        last_activity: None,
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

    fn write(&mut self, opcode: u32, data: Option<&[u8]>) -> io::Result<()> {
        if let Some(packet) = data {
            self.pipe.write_all(
                utils::encode(opcode, packet.len() as u32).as_slice(),
            )?;
            self.pipe.write_all(packet)?;
        } else {
            self.pipe.write_all(utils::encode(opcode, 0).as_slice())?;
        }
        Ok(())
    }

    fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut header = [0; 8];
        self.pipe.read(&mut header)?;
        let mut buffer = vec![0u8; utils::decode(&header) as usize];
        self.pipe.read(&mut buffer)?;
        Ok(buffer)
    }

    fn close(&mut self) -> io::Result<()> {
        self.pipe.write_all(utils::encode(2, 0).as_slice())?;
        self.pipe.flush()?;

        Ok(())
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
            self.write(1, Some(packet.to_json().unwrap().as_bytes()))
        } else {
            Ok(())
        }
    }

    fn clear(&mut self) -> io::Result<()> {
        self.write(1, None)
    }
}
