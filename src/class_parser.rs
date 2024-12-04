use crate::common::{MessageError, Result, ToResult};
use crate::jclass_info::{JClassInfo, LazyValue};
use crate::util::byte_utils::{bytes_to_u16_be, bytes_to_u32_be};
use std::fmt::Debug;
use std::io::Read;

pub type ReadBox = Box<dyn Read>;

pub const JCLASS_MAGIC: [u8;4] = [0xCA, 0xFE, 0xBA, 0xBE];
pub const JCLASS_MAGIC_: u32 = 0xCAFEBABE;

pub struct ClassParser {
    read: ReadBox,
    jclass_info: JClassInfo,
}

// impl Debug for ClassParser {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("JClassInfo")
//             .field("jclass_info", &self.jclass_info)
//             .finish()
//     }
// }

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
                match $var.read_class_bytes_u16($name) {
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
            read: Box::new(read),
            jclass_info: JClassInfo::default(),
        }
    }

    fn read_class_bytes(&mut self, name: &str, bytes: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0;bytes];
        let len = self.read.read(&mut buf).with_message( &format!("{name}读取出错"))?;
        if len < bytes {
            return MessageError::new(&format!("{name}读取出错，文件长度过小")).into();
        }
        Ok(buf)
    }

    fn read_class_bytes_u16(&mut self, name: &str) -> Result<u16> {
        let bytes = self.read_class_bytes(name, 2)?;
        Ok(bytes_to_u16_be(&bytes))
    }

    fn read_class_bytes_u32(&mut self, name: &str) -> Result<u32> {
        let bytes = self.read_class_bytes(name, 4)?;
        Ok(bytes_to_u32_be(&bytes))
    }

    pub fn magic(&mut self) -> Result<u32> {
        class_info_field_get!(self.jclass_info.magic, {
            match self.read_class_bytes_u32("魔术头") {
                Ok(magic_value) => {
                    if magic_value != JCLASS_MAGIC_ {
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
    pub fn constant_pool(&mut self) -> Result<u16> {
        check_and_load_latest!(self, major_version);
        let bytes = self.read_class_bytes("常量池", 2)?;
        let pool_count = bytes_to_u16_be(&bytes);

        Ok(1)
    }

    pub fn get_jclass_info(&self) -> JClassInfo {
        self.jclass_info.clone()
    }
}

pub fn parse() {

}