use std::io::Read;
use crate::attribute_info::AttributeInfo;
use crate::error::{MessageError, Result};
use crate::constant_pool::ConstantPool;
use crate::field_info::FieldInfo;
use crate::jclass_info::{JClassInfo, LazyValue};
use crate::method_info::MethodInfo;
use crate::support::data_reader::{DataReader, ReadToType};

pub const JCLASS_MAGIC: u32 = 0xCAFEBABE;

pub struct ClassParser<T: Read> {
    reader: DataReader<T>,
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
                match ReadToType::<u16>::read_to(&mut $var.reader, $name) {
                    Ok(value) => {
                        LazyValue::Some(value)
                    }
                    Err(e) => LazyValue::Err(e)
                }
            }).to_result($name)
        }
    };
}

macro_rules! check_latest_and_get_mul {
    ($var:expr, $field:ident, $latest_field:ident, $name:expr, $item_get:expr) => {
        {
            check_and_load_latest!($var, $latest_field);
            class_info_field_get!($var.jclass_info.$field, {
                match ReadToType::<u16>::read_to(&mut $var.reader, concat!($name, "数量")) {
                    Ok(size) => {
                        let size = size as usize;
                        let mut values = Vec::with_capacity(size);
                        // let mut failed = false;
                        let mut err = None;
                        for _ in 0..size {
                            match $item_get {
                                Ok(val) => {
                                    values.push(val);
                                }
                                Err(e) => {
                                    err = Some(e);
                                    // failed = true;
                                    break;
                                }
                            }
                        }
                        if let Some(e) = err {
                            LazyValue::Err(e)
                        } else {
                            LazyValue::Some(values)
                        }
                    }
                    Err(_) => {
                        LazyValue::Err(MessageError::new(concat!("读取", $name, "数量失败")))
                    }
                }
            }).to_result($name)
        }
    };
}

impl<T: Read> ClassParser<T> {
    pub fn new(read: DataReader<T>) -> ClassParser<T> {
        ClassParser {
            reader: read,
            jclass_info: JClassInfo::default(),
        }
    }

    pub fn magic(&mut self) -> Result<u32> {
        class_info_field_get!(self.jclass_info.magic, {
            match self.reader.read_to("魔术头") {
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

    pub fn interfaces(&mut self) -> Result<Vec<u16>> {
        check_latest_and_get_mul!(self, interfaces, superclass_index, "接口",
            self.reader.read_to("接口数量"))
    }

    pub fn fields(&mut self) -> Result<Vec<FieldInfo>> {
        check_latest_and_get_mul!(self, fields, interfaces, "字段",
            FieldInfo::new_from_reader(&mut self.reader))
    }

    pub fn methods(&mut self) -> Result<Vec<MethodInfo>> {
        check_latest_and_get_mul!(self, methods, fields, "方法",
            MethodInfo::new_from_reader(&mut self.reader))
    }

    pub fn attributes(&mut self) -> Result<Vec<AttributeInfo>> {
        check_latest_and_get_mul!(self, attributes, methods, "属性",
            match &self.jclass_info.constant_pool {
                LazyValue::Some(pool) => {
                    AttributeInfo::new_from_reader(&mut self.reader, pool)
                }
                _ => {
                    Err(MessageError::new("常量池数据异常"))
                }
            })
    }

    pub fn load_all(&mut self) -> Result<()> {
        self.attributes()?;
        Ok(())
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