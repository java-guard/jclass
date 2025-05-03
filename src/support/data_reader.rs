use crate::common::error::{MessageError, Result};
use std::io::Read;
use std::ops::{Deref, DerefMut};
use crate::with_message;

pub struct DataReader<T: Read> (T);

pub trait ReadToType<T:Sized> {
    fn read_to(&mut self, name: &str) -> Result<T>;
}

impl<T: Read> DataReader<T> {
    #[inline]
    pub fn read_bytes(&mut self, name: &str, bytes: &mut [u8]) -> Result<()> {
        with_message!(self.read_exact(bytes), &format!("{name}读取出错"))
        // self.read_exact(bytes).with_message( &format!("{name}读取出错"))
    }
    #[inline]
    pub fn read_bytes_with_pre_size(&mut self, name: &str) -> Result<Vec<u8>> {
        let str_len: u16 = self.read_to(name)?;
        let mut buf = vec![0; str_len as usize];
        self.read_bytes(name, &mut buf)?;
        Ok(buf)
    }
}

impl<T: Read> From<T> for DataReader<T> {
    fn from(value: T) -> Self {
        DataReader(value)
    }
}

impl<T: Read> Deref for DataReader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Read> DerefMut for DataReader<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Read> ReadToType<u8> for DataReader<T> {
    fn read_to(&mut self, name: &str) -> Result<u8> {
        let mut buf = [0;1];
        self.read_bytes(name, &mut buf)?;
        Ok(buf[0])
    }
}

impl<T: Read> ReadToType<i8> for DataReader<T> {
    #[inline]
    fn read_to(&mut self, name: &str) -> Result<i8> {
        let mut buf = [0;1];
        self.read_bytes(name, &mut buf)?;
        Ok(buf[0] as i8)
    }
}

macro_rules! support_read_by_byte {
    ($($type_name:ident),+ $(,)?) => {
        $(
            impl<T: Read> ReadToType<$type_name> for DataReader<T> {
                fn read_to(&mut self, name: &str) -> Result<$type_name> {
                    let mut buf = [0;size_of::<$type_name>()];
                    self.read_bytes(name, &mut buf)?;
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