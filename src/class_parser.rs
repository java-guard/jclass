use crate::common::{MessageError, Result, ToResult};
use crate::jclass_info::{JClassInfo, LazyValue};
use std::fmt::{Debug, Formatter};
use std::io::Read;

pub type ReadBox = Box<dyn Read>;

pub const JCLASS_MAGIC: [u8;4] = [0xCA, 0xFE, 0xBA, 0xBE];

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

fn read_class_bytes(read: &mut ReadBox, name: &str, bytes: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0;bytes];
    let len = read.read(&mut buf).with_message( &format!("{name}读取出错"))?;
    if len < bytes {
        return MessageError::new(&format!("{name}读取出错，文件长度过小")).into();
    }
    Ok(buf)
}

type ReadClassLazyValueHandler<T> = fn(&[u8]) -> LazyValue<T>;

fn read_class_lazy_value<T: Clone>(read: &mut ReadBox, var: &mut LazyValue<T>, name: &str, bytes: usize,
                            handle: ReadClassLazyValueHandler<T>) -> Result<T> {
    if !var.is_load() {
        match read_class_bytes(read, name, bytes) {
            Ok(buf) => {
                let value = handle(buf.as_slice());
                var.update(value);
            }
            Err(e) => {
                var.err(e);
                // return Err(e)
            }
        }

    }
    // var.clone()
    var.to_result(name)
}

fn read_class_lazy_value_u16(read: &mut ReadBox, var: &mut LazyValue<u16>, name: &str)
                                                                                -> Result<u16> {
    let handle: ReadClassLazyValueHandler<u16> = |buf| {
        let result: Result<[u8;2]> = buf.try_into().with_message("数据转换失败");
        match result {
            Ok(arr) => LazyValue::Some(u16::from_be_bytes(arr)),
            Err(e) => LazyValue::Err(e)
        }
        // let arr:[u8;2] = buf.try_into::<[u8;2]>();
        // LazyValue::Some(u16::from_le_bytes(arr))
    };
    read_class_lazy_value(read, var, name, 2, handle)
}

macro_rules! check_and_load_latest {
    ($var:expr, $field:ident) => {
        if !$var.jclass_info.$field.is_load() {
            $var.$field()?;
        }
    };
}

macro_rules! check_and_load_u16 {
    ($var:expr, $latest_field:ident, $field:ident, $name:expr) => {
        {
            check_and_load_latest!($var, $latest_field);
            read_class_lazy_value_u16(&mut $var.read, &mut $var.jclass_info.$field, $name)
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

    pub fn magic(&mut self) -> Result<[u8;4]> {
        let handler:ReadClassLazyValueHandler<[u8;4]> = |buf| {
            if buf.len() != 4 || buf != JCLASS_MAGIC {
                LazyValue::Err(MessageError::new("解析数据非class文件"))
            } else {
                let result: Result<[u8;4]> = buf.try_into().with_message("魔术头非4字节");
                match result {
                    Ok(arr) => LazyValue::Some(arr),
                    Err(e) => LazyValue::Err(e)
                }

            }
        };
        read_class_lazy_value(&mut self.read, &mut self.jclass_info.magic, "魔术头", 4, handler)
    }
    pub fn minor_version(&mut self) -> Result<u16> {
        check_and_load_u16!(self, magic, minor_version, "次版本")
    }
    pub fn major_version(&mut self) -> Result<u16> {
        check_and_load_u16!(self, minor_version, major_version, "主版本")
    }

    pub fn get_jclass_info(&self) -> JClassInfo {
        self.jclass_info.clone()
    }
}

pub fn parse() {

}