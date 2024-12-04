

pub fn bytes_to_u16_be(bytes: &[u8]) -> u16 {
    match bytes.len() {
        0 => 0,
        1 => u16::from_be_bytes([0, bytes[0]]),
        _ => u16::from_be_bytes([bytes[0], bytes[1]])
    }
}

pub fn bytes_to_u32_be(bytes: &[u8]) -> u32 {
    match bytes.len() {
        0 => 0,
        1 => u32::from_be_bytes([0, 0, 0, bytes[0]]),
        2 => u32::from_be_bytes([0, 0, bytes[0], bytes[1]]),
        3 => u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]),
        _ => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}