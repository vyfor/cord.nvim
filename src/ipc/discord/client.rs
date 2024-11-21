use crate::ipc::discord::utils;
use crate::json::serialize::Serialize;
use crate::json::Json;
use crate::presence::types::{Activity, Packet};
use std::io::{self, Read, Write};

pub struct RichClient {
    pub client_id: u64,
    #[cfg(target_os = "windows")]
    pub pipe: Option<std::fs::File>,
    #[cfg(not(target_os = "windows"))]
    pub pipe: Option<std::os::unix::net::UnixStream>,
    pub last_activity: Option<Activity>,
    pub pid: u32,
}

pub trait Connection {
    fn connect(client_id: u64) -> io::Result<RichClient>;
    fn close(&mut self);
}

impl RichClient {
    pub fn write(&self, opcode: u32, data: Option<&[u8]>) -> io::Result<()> {
        self.pipe.as_ref().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |mut pipe| {
                let payload = match data {
                    Some(packet) => {
                        let mut payload = utils::encode(opcode, packet.len() as u32);
                        payload.extend_from_slice(packet);
                        payload
                    }
                    None => utils::encode(opcode, 0),
                };
                pipe.write_all(&payload)
            },
        )
    }

    pub fn read(&self) -> io::Result<Vec<u8>> {
        self.pipe.as_ref().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |mut pipe| {
                let mut header = [0; 8];
                pipe.read_exact(&mut header)?;
                let size = utils::decode(&header) as usize;
                let mut buffer = vec![0u8; size];
                pipe.read_exact(&mut buffer)?;
                Ok(buffer)
            },
        )
    }

    pub fn handshake(&self) -> io::Result<()> {
        self.write(
            0,
            Some(format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id).as_bytes()),
        )
    }

    pub fn update(&self, packet: &Packet) -> Result<(), Box<dyn std::error::Error>> {
        if packet.activity != self.last_activity {
            let encoded = Json::serialize(packet)?;

            self.write(1, Some(encoded.as_bytes()))?;
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let packet = Packet {
            pid: self.pid,
            activity: None,
        };
        let encoded = Json::serialize(&packet)?;

        self.write(1, Some(encoded.as_bytes()))?;

        Ok(())
    }
}
