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

// macro_rules! bytes_to_be {
//     ($bytes:expr, $to_type:expr) => {
//         {
//             let size = size_of::<$to_type>();
//             let bytes_len = $bytes.len();
//             match bytes_len {
//                 l if l < size => {
//                     let mut arr = [0;size_of::<$to_type>()];
//                     arr[size - bytes_len..bytes_len].copy_from_slice($bytes);
//                     $to_type::from_be_bytes(arr)
//                 }
//                 _ => {
//                     $to_type::from_be_bytes($bytes.try_into().unwrap())
//                 }
//             }
//         }
//     };
// }

// pub fn bytes_to_be(bytes: &[u8], be: bool) -> i32 {
//     i32::fromb
//     bytes_to_be!()
// }