use std::convert::TryInto;

/// Combines an opcode and data length into a byte vector for transmission.
pub fn encode(opcode: u32, data_length: u32) -> Vec<u8> {
    [opcode.to_le_bytes(), data_length.to_le_bytes()].concat()
}

/// Extracts the opcode and data length from a byte slice.
/// Returns None if the slice is too short.
/// Returns Some(opcode, length) if successful.
pub fn decode(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 8 {
        return None;
    }
    let opcode = data[..4].try_into().map(u32::from_le_bytes).ok()?;
    let length = data[4..8].try_into().map(u32::from_le_bytes).ok()?;
    Some((opcode, length))
}
