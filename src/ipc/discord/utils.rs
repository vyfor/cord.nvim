use std::convert::TryInto;

/// Combines an opcode and data length into a byte vector for transmission.
pub fn encode(opcode: u32, data_length: u32) -> Vec<u8> {
    [opcode.to_le_bytes(), data_length.to_le_bytes()].concat()
}

/// Extracts the data length from a byte slice.
pub fn decode(data: &[u8]) -> u32 {
    u32::from_le_bytes(data[4..8].try_into().unwrap())
}
