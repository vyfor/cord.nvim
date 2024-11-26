use crate::ipc::discord::utils;
use crate::json::Json;
use crate::presence::types::Packet;
use std::io::{Read, Write};
use std::sync::atomic::AtomicBool;

pub struct RichClient {
    pub client_id: u64,
    #[cfg(target_os = "windows")]
    pub pipe: Option<std::fs::File>,
    #[cfg(not(target_os = "windows"))]
    pub pipe: Option<std::os::unix::net::UnixStream>,
    pub pid: u32,
    pub is_ready: AtomicBool,
}

pub trait Connection {
    fn connect(client_id: u64) -> crate::Result<RichClient>;
    fn close(&mut self);
}

impl RichClient {
    pub fn write(&self, opcode: u32, data: Option<&[u8]>) -> crate::Result<()> {
        self.pipe
            .as_ref()
            .map_or(Err("Pipe not found".into()), |mut pipe| {
                let payload = match data {
                    Some(packet) => {
                        let mut payload = utils::encode(opcode, packet.len() as u32);
                        payload.extend_from_slice(packet);
                        payload
                    }
                    None => utils::encode(opcode, 0),
                };
                pipe.write_all(&payload)?;

                Ok(())
            })
    }

    pub fn read(&self) -> crate::Result<Vec<u8>> {
        self.pipe
            .as_ref()
            .map_or(Err("Pipe not found".into()), |mut pipe| {
                let mut header = [0; 8];
                pipe.read_exact(&mut header)?;
                let size = utils::decode(&header) as usize;
                let mut buffer = vec![0u8; size];
                pipe.read_exact(&mut buffer)?;
                Ok(buffer)
            })
    }

    pub fn handshake(&self) -> crate::Result<()> {
        self.write(
            0,
            Some(format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id).as_bytes()),
        )
    }

    pub fn update(&self, packet: &Packet) -> crate::Result<()> {
        let encoded = Json::serialize(packet)?;
        self.write(1, Some(encoded.as_bytes()))?;

        Ok(())
    }

    pub fn clear(&self) -> crate::Result<()> {
        let packet = Packet::empty();
        let encoded = Json::serialize(&packet)?;

        self.write(1, Some(encoded.as_bytes()))?;

        Ok(())
    }
}
