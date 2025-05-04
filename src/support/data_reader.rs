use crate::common::error::{MessageError, Result};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use crate::with_message;

pub struct DataReader<T: Read> (T);
pub struct DataWriter<T: Write> (T);

pub trait ReadToType<T:Sized> {
    fn read_to(&mut self, name: &str) -> Result<T>;
}

pub trait WriteFromType<T:Sized> {
    fn write_from(&mut self, name: &str, data: T) -> Result<()>;
}

impl<T: Read> DataReader<T> {
    #[inline]
    pub fn read_bytes(&mut self, name: &str, bytes: &mut [u8]) -> Result<()> {
        with_message!(self.read_exact(bytes), &format!("{name}读取出错"))
    }
    #[inline]
    pub fn read_bytes_with_pre_size(&mut self, name: &str) -> Result<Vec<u8>> {
        let str_len: u16 = self.read_to(name)?;
        let str_len = str_len as usize;
        let mut buf = Vec::with_capacity(str_len);
        unsafe {
            buf.set_len(str_len);
        }
        self.read_bytes(name, &mut buf)?;
        Ok(buf)
    }
}

impl<T: Read> From<T> for DataReader<T> {
    fn from(value: T) -> Self {
        DataReader (value)
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
    #[inline]
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
                #[inline]
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



impl<T: Write> From<T> for DataWriter<T> {
    fn from(value: T) -> Self {
        DataWriter (value)
    }
}

impl<T: Write> Deref for DataWriter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Write> DerefMut for DataWriter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Write> DataWriter<T> {
    #[inline]
    pub fn write_bytes(&mut self, name: &str, data: &[u8]) -> Result<()> {
        with_message!(self.write_all(data), &format!("{name}写出出错"))
    }
    #[inline]
    pub fn write_bytes_with_pre_size(&mut self, name: &str, data: &[u8]) -> Result<()> {
        with_message!(self.write_all(&(data.len() as u16).to_be_bytes()), &format!("{name}长度写出出错"))?;
        with_message!(self.write_all(data), &format!("{name}写出出错"))
    }
}


impl<T: Write> WriteFromType<u8> for DataWriter<T> {
    #[inline]
    fn write_from(&mut self, name: &str, data: u8) -> Result<()> {
        self.write_bytes(name, &[data])
    }
}

impl<T: Write> WriteFromType<i8> for DataWriter<T> {
    #[inline]
    fn write_from(&mut self, name: &str, data: i8) -> Result<()> {
        self.write_bytes(name, &[data as u8])
    }
}

macro_rules! support_write_to_byte {
    ($($type_name:ident),+ $(,)?) => {
        $(
            impl<T: Write> WriteFromType<$type_name> for DataWriter<T> {
                #[inline]
                fn write_from(&mut self, name: &str, data: $type_name) -> Result<()> {
                    self.write_bytes(name, &data.to_be_bytes())
                }
            }
        )+
    };
}

support_write_to_byte!{
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