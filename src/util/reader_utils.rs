use crate::common::{Reader, Result, ToResult};
use std::io::Read;

pub fn read_bytes<T: Read>(reader: &mut T, name: &str, bytes: &mut [u8]) -> Result<()> {
    reader.read_exact(bytes).with_message( &format!("{name}读取出错"))?;
    // if len < bytes.len() {
    //     return MessageError::new(&format!("{name}读取出错，文件长度过小:{len},{}", bytes.len())).into();
    // }
    Ok(())
}
pub fn read_bytes_with_pre_size(reader: &mut Reader, name: &str) -> Result<Vec<u8>> {
    let str_len: u16 = reader.read_to(name)?;
    let mut buf = vec![0; str_len as usize];
    read_bytes(reader, name, &mut buf)?;
    Ok(buf)
}

pub trait ReadToType<T:Sized> {
    fn read_to(&mut self, name: &str) -> Result<T>;
}

impl ReadToType<u8> for Reader {
    fn read_to(&mut self, name: &str) -> Result<u8> {
        let mut buf = [0;1];
        read_bytes(self, name, &mut buf)?;
        Ok(buf[0])
    }
}

impl ReadToType<i8> for Reader {
    fn read_to(&mut self, name: &str) -> Result<i8> {
        let mut buf = [0;1];
        read_bytes(self, name, &mut buf)?;
        Ok(buf[0] as i8)
    }
}

macro_rules! support_read_by_byte {
    ($($type_name:ident),+ $(,)?) => {
        $(
            impl ReadToType<$type_name> for Reader {
                fn read_to(&mut self, name: &str) -> Result<$type_name> {
                    let mut buf = [0;size_of::<$type_name>()];
                    read_bytes(self, name, &mut buf)?;
                    Ok($type_name::from_be_bytes(buf))
                }
            }
        )+
    };
}

support_read_by_byte!{
    i16,
    u16,
    u32,
    i32,
    u64,
    i64,
    f32,
    f64,
    usize,
}