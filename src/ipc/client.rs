use crate::rpc::packet::Activity;

#[cfg(target_os = "windows")]
pub struct RichClient {
    pub client_id: u64,
    pub pipe: Option<std::fs::File>,
    pub last_activity: Option<Activity>,
}

#[cfg(not(target_os = "windows"))]
pub struct RichClient {
    pub client_id: u64,
    pub pipe: Option<std::os::unix::net::UnixStream>,
    pub last_activity: Option<Activity>,
}

pub trait Connection {
    fn connect(
        client_id: u64,
    ) -> Result<RichClient, Box<dyn std::error::Error>>;
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn write(
        &mut self,
        opcode: u32,
        data: Option<&[u8]>,
    ) -> std::io::Result<()>;
    fn close(&mut self);
    fn handshake(&mut self) -> std::io::Result<()>;
    fn update(
        &mut self,
        packet: &crate::rpc::packet::Packet,
    ) -> std::io::Result<()>;
    fn clear(&mut self) -> std::io::Result<()>;
}
