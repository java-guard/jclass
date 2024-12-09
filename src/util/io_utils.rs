use crate::common::{MessageError, Result, ToResult};
use crate::util::byte_utils::{bytes_to_u16_be, bytes_to_u32_be};
use std::io::Read;

pub fn read_class_bytes<T: Read>(reader: &mut T, name: &str, bytes: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0;bytes];
    let len = reader.read(&mut buf).with_message( &format!("{name}读取出错"))?;
    if len < bytes {
        return MessageError::new(&format!("{name}读取出错，文件长度过小")).into();
    }
    Ok(buf)
}

pub fn read_class_bytes_u16<T: Read>(reader: &mut T, name: &str) -> Result<u16> {
    let bytes = read_class_bytes(reader, name, 2)?;
    Ok(bytes_to_u16_be(&bytes))
}

pub fn read_class_bytes_u32<T: Read>(reader: &mut T, name: &str) -> Result<u32> {
    let bytes = read_class_bytes(reader, name, 4)?;
    Ok(bytes_to_u32_be(&bytes))
}