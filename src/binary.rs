pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
    (bytes[0] as i16) << 8 | bytes[1] as i16
}

pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
