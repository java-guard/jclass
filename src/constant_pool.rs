use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8};
use crate::common::{MessageError, Reader, Result, ToResult};
use crate::util::reader_utils::{read_bytes_with_pre_size, ReadToType};
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
    pub fn new_with_reader(reader: &mut Reader) -> Result<RefInfo> {
        let class_index: u16 = reader.read_to("ref: class index 值读取失败")?;
        let name_type_index: u16 = reader.read_to("ref: name type index 值读取失败")?;
        Ok(RefInfo::new(class_index, name_type_index))
    }
}

impl ConstantValue {
    pub fn new_with_reader(reader: &mut Reader) -> Result<ConstantValue> {
        let mut single_byte = [0;1];
        let len = reader.read(&mut single_byte).with_message("常量类型读取出错")?;
        if len != 1 {
            return Err(MessageError::new("常量类型无法读取"));
        }
        Ok(match single_byte[0] as i32 {
            JVM_CONSTANT_Class => {
                ConstantValue::ConstantClass(reader.read_to("Class常量")?)
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
                ConstantValue::ConstantString(reader.read_to("String常量")?)
            }
            JVM_CONSTANT_Integer => {
                ConstantValue::ConstantInteger(reader.read_to("Integer常量")?)
            }
            JVM_CONSTANT_Float => {
                ConstantValue::ConstantFloat(reader.read_to("Float常量")?)
            }
            JVM_CONSTANT_Long => {
                ConstantValue::ConstantLong(reader.read_to("Long常量")?)
            }
            JVM_CONSTANT_Double => {
                ConstantValue::ConstantDouble(reader.read_to("Double常量")?)
            }
            JVM_CONSTANT_NameAndType => {
                let name_index: u16 = reader.read_to("名字和描述符常量")?;
                let type_index: u16 = reader.read_to("名字和描述符常量")?;
                ConstantValue::ConstantNameAndType(name_index, type_index)
            }
            JVM_CONSTANT_Utf8 => {
                // todo 记得删
                // let now = Instant::now();
                let str_bytes = read_bytes_with_pre_size(reader, "UTF8字符串常量")?;
                // println!(">>>> str b : {:?}", now.elapsed());
                // let now = Instant::now();
                let string = String::from_utf8(str_bytes).with_message("UTF8常量读取出错")?;
                // println!(">>>> str : {:?}", now.elapsed());
                ConstantValue::ConstantUtf8(string)
            }
            JVM_CONSTANT_MethodHandle => {
                let kind_byte: u8 = reader.read_to("Method Handle常量")?;
                let ref_index: u16 = reader.read_to("Method Handle常量",)?;
                ConstantValue::ConstantMethodHandle(kind_byte, ref_index)
            }
            JVM_CONSTANT_MethodType => {
                ConstantValue::ConstantMethodType(reader.read_to("MethodType常量")?)
            }
            JVM_CONSTANT_Dynamic => {
                let bootstrap: u16 = reader.read_to("Dynamic常量",)?;
                let name_and_type_index: u16 = reader.read_to("Dynamic常量",)?;
                ConstantValue::ConstantDynamic(bootstrap, name_and_type_index)
            }
            JVM_CONSTANT_InvokeDynamic => {
                let bootstrap: u16 = reader.read_to("InvokeDynamic常量")?;
                let name_and_type_index: u16 = reader.read_to("InvokeDynamic常量")?;
                ConstantValue::ConstantInvokeDynamic(bootstrap, name_and_type_index)
            }
            JVM_CONSTANT_Module => {
                let index: u16 = reader.read_to("模块名常量")?;
                ConstantValue::ConstantModule(index)
            }
            JVM_CONSTANT_Package => {
                let index: u16 = reader.read_to("包名常量")?;
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
    pub fn new_with_reader(reader: &mut Reader) -> Result<ConstantPool> {
        // let now = Instant::now();
        let name = "常量池";
        let pool_count: u16 = reader.read_to(name)?;
        let mut pool = ConstantPool::new(pool_count);
        for _ in 1..pool_count {
            // let now = Instant::now();
            let value = ConstantValue::new_with_reader(reader)?;
            // println!(">>>> pool item: {:?}: {:?}", now.elapsed(), &value);
            pool.add_constant(value);
        }
        // println!(">>> pool: {:?}", now.elapsed());
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