use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8};
use crate::common::{MessageError, Result, ToResult};
use crate::util::byte_utils::bytes_to_u16_be;
use crate::util::io_utils::{read_class_bytes, read_class_bytes_u16};
use std::io::Read;

#[derive(Clone, Debug)]
pub struct RefInfo {
    class_index: u16,
    name_type_index: u16,
}

#[derive(Clone, Debug)]
pub enum ConstantValue {
    Null,
    // name index
    ConstantClass(u16),
    ConstantFieldref(RefInfo),
    ConstantMethodref(RefInfo),
    ConstantInterfaceMethodref(RefInfo),
    // index
    ConstantString(u16),
    ConstantInteger(i32),
    ConstantFloat(f32),
    ConstantLong(i64),
    ConstantDouble(f64),
    // name index, type index
    ConstantNameAndType(u16, u16),
    ConstantUtf8(String),
    // ref kind, ref index
    ConstantMethodHandle(u8, u16),
    // descriptor
    ConstantMethodType(u16),
    // bootstrap method, ref index
    ConstantDynamic(u16, u16),
    // ConstantDynamicCallSite,
    ConstantInvokeDynamic(u16, u16),
    // name index
    ConstantModule(u16),
    // name index
    ConstantPackage(u16),
}

#[derive(Debug, Clone)]
pub struct ConstantItem {
    index: u16,
    value: ConstantValue
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    count: u16,
    values: Vec<ConstantItem>
}

impl RefInfo {
    pub fn new(class_index: u16, name_type_index: u16) -> RefInfo {
        RefInfo {
            class_index,
            name_type_index
        }
    }
    pub fn new_with_reader<T: Read>(reader: &mut T) -> Result<RefInfo> {
        let class_index = read_class_bytes_u16(reader, "ref: class index 值读取失败")?;
        let name_type_index = read_class_bytes_u16(reader, "ref: name type index 值读取失败")?;
        Ok(RefInfo::new(class_index, name_type_index))
    }
}

impl ConstantValue {
    pub fn new_with_reader<T: Read>(reader: &mut T) -> Result<ConstantValue> {
        let mut single_byte = [0;1];
        let len = reader.read(&mut single_byte).with_message("常量类型读取出错")?;
        if len != 1 {
            return Err(MessageError::new("常量类型无法读取"));
        }
        Ok(match single_byte[0] as i32 {
            JVM_CONSTANT_Class => {
                ConstantValue::ConstantClass(read_class_bytes_u16(reader, "Class常量")?)
            }
            JVM_CONSTANT_Fieldref => {
                ConstantValue::ConstantFieldref(RefInfo::new_with_reader(reader)?)
            }
            JVM_CONSTANT_Methodref => {
                ConstantValue::ConstantMethodref(RefInfo::new_with_reader(reader)?)
            }
            JVM_CONSTANT_InterfaceMethodref => {
                ConstantValue::ConstantInterfaceMethodref(RefInfo::new_with_reader(reader)?)
            }
            JVM_CONSTANT_String => {
                ConstantValue::ConstantString(read_class_bytes_u16(reader, "String常量")?)
            }
            JVM_CONSTANT_Integer => {
                let bytes = read_class_bytes(reader, "Integer常量", 4)?;
                // Ok(i32::from_be_bytes(bytes.try_into().unwrap()))
                ConstantValue::ConstantInteger(i32::from_be_bytes(bytes.try_into().unwrap()))
                // ConstantValue::ConstantInteger(read_class_bytes_u32(reader, "Class常量")? as i32)
            }
            JVM_CONSTANT_Float => {
                let bytes = read_class_bytes(reader, "Float常量", 4)?;
                ConstantValue::ConstantFloat(f32::from_be_bytes(bytes.try_into().unwrap()))
            }
            JVM_CONSTANT_Long => {
                let bytes = read_class_bytes(reader, "Long常量", 8)?;
                ConstantValue::ConstantLong(i64::from_be_bytes(bytes.try_into().unwrap()))
            }
            JVM_CONSTANT_Double => {
                let bytes = read_class_bytes(reader, "Double常量", 8)?;
                ConstantValue::ConstantDouble(f64::from_be_bytes(bytes.try_into().unwrap()))
            }
            JVM_CONSTANT_NameAndType => {
                let name_index = read_class_bytes_u16(reader, "名字和描述符常量")?;
                let type_index = read_class_bytes_u16(reader, "名字和描述符常量")?;
                ConstantValue::ConstantNameAndType(name_index, type_index)
            }
            JVM_CONSTANT_Utf8 => {
                let str_len = read_class_bytes_u16(reader, "UTF8字符串常量")?;
                let str_bytes = read_class_bytes(reader, "UTF8字符串常量", str_len as usize)?;
                ConstantValue::ConstantUtf8(String::from_utf8(str_bytes).with_message("UTF8常量读取出错")?)
            }
            JVM_CONSTANT_MethodHandle => {
                let kind_byte = read_class_bytes(reader, "Method Handle常量", 1)?;
                let ref_index = read_class_bytes_u16(reader, "Method Handle常量",)?;
                ConstantValue::ConstantMethodHandle(kind_byte[0], ref_index)
            }
            JVM_CONSTANT_MethodType => {
                ConstantValue::ConstantMethodType(read_class_bytes_u16(reader, "MethodType常量")?)
            }
            JVM_CONSTANT_Dynamic => {
                let bootstrap = read_class_bytes_u16(reader, "Dynamic常量",)?;
                let name_and_type_index = read_class_bytes_u16(reader, "Dynamic常量",)?;
                ConstantValue::ConstantDynamic(bootstrap, name_and_type_index)
            }
            JVM_CONSTANT_InvokeDynamic => {
                let bootstrap = read_class_bytes_u16(reader, "InvokeDynamic常量")?;
                let name_and_type_index = read_class_bytes_u16(reader, "InvokeDynamic常量")?;
                ConstantValue::ConstantInvokeDynamic(bootstrap, name_and_type_index)
            }
            JVM_CONSTANT_Module => {
                let index = read_class_bytes_u16(reader, "模块名常量")?;
                ConstantValue::ConstantModule(index)
            }
            JVM_CONSTANT_Package => {
                let index = read_class_bytes_u16(reader, "包名常量")?;
                ConstantValue::ConstantPackage(index)
            }
            _ => {
                return Err(MessageError::new(&format!("无效的常量类型[{}]", single_byte[0])));
            }
        })
    }
    pub fn byte(&self) -> u8 {
        let result = match self {
            ConstantValue::Null => 0,
            ConstantValue::ConstantClass(_) => JVM_CONSTANT_Class,
            ConstantValue::ConstantFieldref(_) => JVM_CONSTANT_Fieldref,
            ConstantValue::ConstantMethodref(_) => JVM_CONSTANT_Methodref,
            ConstantValue::ConstantInterfaceMethodref(_) => JVM_CONSTANT_InterfaceMethodref,
            ConstantValue::ConstantString(_) => JVM_CONSTANT_String,
            ConstantValue::ConstantInteger(_) => JVM_CONSTANT_Integer,
            ConstantValue::ConstantFloat(_) => JVM_CONSTANT_Float,
            ConstantValue::ConstantLong(_) => JVM_CONSTANT_Long,
            ConstantValue::ConstantDouble(_) => JVM_CONSTANT_Double,
            ConstantValue::ConstantNameAndType(_, _) => JVM_CONSTANT_NameAndType,
            ConstantValue::ConstantUtf8(_) => JVM_CONSTANT_Utf8,
            ConstantValue::ConstantMethodHandle(_, _) => JVM_CONSTANT_MethodHandle,
            ConstantValue::ConstantMethodType(_) => JVM_CONSTANT_MethodType,
            ConstantValue::ConstantDynamic(_, _) => JVM_CONSTANT_Dynamic,
            // ConstantValue::ConstantDynamicCallSite => JVM_CONSTANT_InvokeDynamic,
            ConstantValue::ConstantInvokeDynamic(_, _) => JVM_CONSTANT_InvokeDynamic,
            ConstantValue::ConstantModule(_) => JVM_CONSTANT_Module,
            ConstantValue::ConstantPackage(_) => JVM_CONSTANT_Package,
            // _ => panic!("未知常量值类型")
        };
        result as u8
    }
}

impl ConstantPool {
    pub fn new(size: u16) -> ConstantPool {
        let mut pool = ConstantPool {
            count: 0,
            values: Vec::with_capacity(size as usize)
        };
        pool.values.push(ConstantItem {
            index: 0,
            value: ConstantValue::Null
        });
        pool
    }
    pub fn new_with_reader<T: Read>(reader: &mut T) -> Result<ConstantPool> {
        let name = "常量池";
        let bytes = read_class_bytes(reader, name, 2)?;
        let pool_count = bytes_to_u16_be(&bytes);
        let mut pool = ConstantPool::new(pool_count);
        for _ in 1..pool_count {
            let value = ConstantValue::new_with_reader(reader)?;
            pool.add_constant(value);
        }
        Ok(pool)
    }

    // pub fn add_constant_item(&mut self, item: ConstantItem) -> u16 {
    //     self.count += 1;
    //     self.values.push(item);
    //     self.count
    // }

    pub fn add_constant(&mut self, value: ConstantValue) -> u16 {
        self.count += 1;
        self.values.push(ConstantItem {
            index: self.count,
            value
        });
        self.count
    }
}