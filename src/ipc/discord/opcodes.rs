/// Discord IPC opcodes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Handshake = 0,
    Frame = 1,
    Close = 2,
    Ping = 3,
    Pong = 4,
}

impl From<u32> for Opcode {
    fn from(code: u32) -> Self {
        match code {
            0 => Opcode::Handshake,
            1 => Opcode::Frame,
            2 => Opcode::Close,
            3 => Opcode::Ping,
            4 => Opcode::Pong,
            _ => Opcode::Frame,
        }
    }
}

impl From<Opcode> for u32 {
    fn from(op: Opcode) -> Self {
        op as u32
    }
}
