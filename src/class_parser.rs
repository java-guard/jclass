use crate::common::{MessageError, Result};
use crate::constant_pool::ConstantPool;
use crate::jclass_info::{JClassInfo, LazyValue};
use crate::util::io_utils::{read_class_bytes_u16, read_class_bytes_u32};
use std::io::Read;

pub type Reader = Box<dyn Read>;

pub const JCLASS_MAGIC: u32 = 0xCAFEBABE;

pub struct ClassParser {
    reader: Reader,
    jclass_info: JClassInfo,
}

macro_rules! check_and_load_latest {
    ($var:expr, $field:ident) => {
        if !$var.jclass_info.$field.is_load() {
            $var.$field()?;
        } else if $var.jclass_info.$field.is_err() {
            $var.jclass_info.$field.to_result(stringify!($field))?;
        }
    };
}

macro_rules! class_info_field_get {
    ($value:expr, $code:block) => {
        match &$value {
            LazyValue::UnLoad => {
                let value = $code;
                $value.update(value.clone());
                value
            }
            _ => $value.clone()
        }
    };
}

macro_rules! check_latest_and_get {
    ($var:expr, $field:ident, $latest_field:ident, $name:expr) => {
        {
            check_and_load_latest!($var, $latest_field);
            class_info_field_get!($var.jclass_info.$field, {
                match read_class_bytes_u16(&mut $var.reader, $name) {
                    Ok(value) => {
                        LazyValue::Some(value)
                    }
                    Err(e) => LazyValue::Err(e)
                }
            }).to_result($name)
        }
    };
}

impl ClassParser {
    pub fn new<T: Read + 'static>(read: T) -> ClassParser {
        ClassParser {
            reader: Box::new(read),
            jclass_info: JClassInfo::default(),
        }
    }

    pub fn magic(&mut self) -> Result<u32> {
        class_info_field_get!(self.jclass_info.magic, {
            match read_class_bytes_u32(&mut self.reader, "魔术头") {
                Ok(magic_value) => {
                    if magic_value != JCLASS_MAGIC {
                        LazyValue::Err(MessageError::new("解析数据非class文件"))
                    } else {
                        LazyValue::Some(magic_value)
                    }
                }
                Err(e) => LazyValue::Err(e)
            }
        }).to_result("魔术头")
    }
    pub fn minor_version(&mut self) -> Result<u16> {
        check_latest_and_get!(self, minor_version, magic, "次版本")
    }
    pub fn major_version(&mut self) -> Result<u16> {
        check_latest_and_get!(self, major_version, minor_version, "主版本")
    }
    pub fn constant_pool(&mut self) -> Result<ConstantPool> {
        check_and_load_latest!(self, major_version);
        class_info_field_get!(self.jclass_info.constant_pool, {
                match ConstantPool::new_with_reader(&mut self.reader) {
                    Ok(pool) => LazyValue::Some(pool),
                    Err(e) => LazyValue::Err(e)
                }
            }).to_result("常量池")
    }

    pub fn access_flags(&mut self) -> Result<u16> {
        check_latest_and_get!(self, access_flags, constant_pool, "访问标志")
    }

    pub fn class_index(&mut self) -> Result<u16> {
        check_latest_and_get!(self, class_index, access_flags, "该类索引")
    }

    pub fn superclass_index(&mut self) -> Result<u16> {
        check_latest_and_get!(self, superclass_index, class_index, "父类索引")
    }

    fn interfaces_read(&mut self) -> LazyValue<Vec<u16>> {
        match read_class_bytes_u16(&mut self.reader, "接口数量") {
            Ok(size) => {
                // if size == 0 {
                //     return LazyValue::None;
                // }
                let size = size as usize;
                let mut interfaces = Vec::with_capacity(size);
                for _ in 0..size {
                    match read_class_bytes_u16(&mut self.reader, "接口索引") {
                        Ok(index) => {
                            interfaces.push(index);
                        }
                        Err(_) => {
                            return LazyValue::Err(MessageError::new("读取接口索引失败"))
                        }
                    }
                }
                LazyValue::Some(interfaces)
            }
            Err(_) => {
                LazyValue::Err(MessageError::new("读取接口数量失败"))
            }
        }
    }

    pub fn interfaces(&mut self) -> Result<Vec<u16>> {
        check_and_load_latest!(self, superclass_index);
        class_info_field_get!(self.jclass_info.interfaces, {self.interfaces_read()}).to_result("接口")
    }

    pub fn get_jclass_info(&self) -> &JClassInfo {
        &self.jclass_info
    }

    pub fn get_jclass_info_mut(&mut self) -> &JClassInfo {
        &mut self.jclass_info
    }
}

// pub fn parse() {
//
// }