use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8};
use crate::common::error::{MessageError, Result};
use crate::support::data_reader::{DataReader, DataWriter, ReadToType, WriteFromType};
use crate::with_message;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use crate::jclass_info::JClassInfo;

#[derive(Clone, Debug)]
pub struct RefInfo {
    class_index: u16,
    name_type_index: u16,
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum ConstantValue {
    Null = 0,
    // name index
    ConstantClass(u16) = JVM_CONSTANT_Class as u8,
    ConstantFieldref(u16, u16) = JVM_CONSTANT_Fieldref as u8,
    ConstantMethodref(u16, u16) = JVM_CONSTANT_Methodref as u8,
    ConstantInterfaceMethodref(u16, u16) = JVM_CONSTANT_InterfaceMethodref as u8,
    // index
    ConstantString(u16) = JVM_CONSTANT_String as u8,
    ConstantInteger(i32) = JVM_CONSTANT_Integer as u8,
    ConstantFloat(f32) = JVM_CONSTANT_Float as u8,
    ConstantLong(i64) = JVM_CONSTANT_Long as u8,
    ConstantDouble(f64) = JVM_CONSTANT_Double as u8,
    // name index, type index
    ConstantNameAndType(u16, u16) = JVM_CONSTANT_NameAndType as u8,
    ConstantUtf8(String) = JVM_CONSTANT_Utf8 as u8,
    // ref kind, ref index
    ConstantMethodHandle(u8, u16) = JVM_CONSTANT_MethodHandle as u8,
    // descriptor
    ConstantMethodType(u16) = JVM_CONSTANT_MethodType as u8,
    // bootstrap method, ref index
    ConstantDynamic(u16, u16) = JVM_CONSTANT_Dynamic as u8,
    // ConstantDynamicCallSite,
    ConstantInvokeDynamic(u16, u16) = JVM_CONSTANT_InvokeDynamic as u8,
    // name index
    ConstantModule(u16) = JVM_CONSTANT_Module as u8,
    // name index
    ConstantPackage(u16) = JVM_CONSTANT_Package as u8,
}

#[derive(Debug, Clone)]
pub struct ConstantItem {
    index: u16,
    value: ConstantValue
}

#[derive(Debug, Clone, Default)]
pub struct ConstantPool {
    count: u16,
    values: Vec<ConstantItem>,
    cache: Option<HashMap<ConstantValue, u16>>
}

impl RefInfo {
    pub fn new(class_index: u16, name_type_index: u16) -> RefInfo {
        RefInfo {
            class_index,
            name_type_index
        }
    }
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<RefInfo> {
        let class_index: u16 = reader.read_to("ref: class index 值读取失败")?;
        let name_type_index: u16 = reader.read_to("ref: name type index 值读取失败")?;
        Ok(RefInfo::new(class_index, name_type_index))
    }
}

impl ConstantValue {
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<ConstantValue> {
        let mut const_type:u8 = reader.read_to("常量类型")?;
        Ok(match const_type.into() {
            0 => {
                ConstantValue::Null
            }
            JVM_CONSTANT_Class => {
                ConstantValue::ConstantClass(reader.read_to("Class常量")?)
            }
            JVM_CONSTANT_Fieldref => {
                ConstantValue::ConstantFieldref(reader.read_to("ConstantField ref: class index 值")?,
                                                reader.read_to("ConstantField ref: name index 值")?)
            }
            JVM_CONSTANT_Methodref => {
                ConstantValue::ConstantMethodref(reader.read_to("ConstantField ref: class index 值")?,
                                                reader.read_to("ConstantField ref: name index 值")?)
            }
            JVM_CONSTANT_InterfaceMethodref => {
                ConstantValue::ConstantInterfaceMethodref(reader.read_to("ConstantField ref: class index 值")?,
                                                reader.read_to("ConstantField ref: name index 值")?)
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
                let str_bytes = reader.read_bytes_with_pre_size("UTF8字符串常量")?;
                let string = with_message!(String::from_utf8(str_bytes), "UTF8常量读取出错")?;
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
                return Err(MessageError::new(&format!("无效的常量类型[{}]", const_type)));
            }
        })
    }

    pub fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        let const_type = self.byte();
        writer.write_from("常量类型", const_type)?;
        match self {
            ConstantValue::Null => {
                Ok(())
            }
            ConstantValue::ConstantClass(class_index) => {
                writer.write_from("Class常量", *class_index)
            }
            ConstantValue::ConstantFieldref(class_index, name_index) => {
                writer.write_from("ConstantField ref: class index 值", *class_index)?;
                writer.write_from("ConstantField ref: name index 值", *name_index)
            }
            ConstantValue::ConstantMethodref(class_index, name_index) => {
                writer.write_from("ConstantField ref: class index 值", *class_index)?;
                writer.write_from("ConstantField ref: name index 值", *name_index)
            }
            ConstantValue::ConstantInterfaceMethodref(class_index, name_index) => {
                writer.write_from("ConstantField ref: class index 值", *class_index)?;
                writer.write_from("ConstantField ref: name index 值", *name_index)
            }
            ConstantValue::ConstantString(utf8_index) => {
                writer.write_from("String常量", *utf8_index)
            }
            ConstantValue::ConstantInteger(val) => {
                writer.write_from("Integer常量", *val)
            }
            ConstantValue::ConstantFloat(val) => {
                writer.write_from("Float常量", *val)
            }
            ConstantValue::ConstantLong(val) => {
                writer.write_from("Long常量", *val)
            }
            ConstantValue::ConstantDouble(val) => {
                writer.write_from("Double常量", *val)
            }
            ConstantValue::ConstantNameAndType(name_index, type_index) => {
                writer.write_from("名字和描述符常量", *name_index)?;
                writer.write_from("名字和描述符常量", *type_index)
            }
            ConstantValue::ConstantUtf8(val) => {
                writer.write_bytes_with_pre_size("UTF8字符串常量", val.as_bytes())
            }
            ConstantValue::ConstantMethodHandle(kind_byte, ref_index) => {
                writer.write_from("Method Handle常量", *kind_byte)?;
                writer.write_from("Method Handle常量", *ref_index)
            }
            ConstantValue::ConstantMethodType(index) => {
                writer.write_from("MethodType常量", *index)
            }
            ConstantValue::ConstantDynamic(bootstrap, name_and_type_index) => {
                writer.write_from("Dynamic常量", *bootstrap)?;
                writer.write_from("Dynamic常量", *name_and_type_index)
            }
            ConstantValue::ConstantInvokeDynamic(bootstrap, name_and_type_index) => {
                writer.write_from("InvokeDynamic常量", *bootstrap)?;
                writer.write_from("InvokeDynamic常量", *name_and_type_index)
            }
            ConstantValue::ConstantModule(index) => {
                writer.write_from("模块名常量", *index)
            }
            ConstantValue::ConstantPackage(index) => {
                writer.write_from("包名常量", *index)
            }
        }
    }

    pub fn byte(&self) -> u8 {
        unsafe {
            *(self as *const ConstantValue as *const u8)
        }
    }
    // pub fn byte(&self) -> u8 {
    //     let result = match self {
    //         ConstantValue::Null => 0,
    //         ConstantValue::ConstantClass(_) => JVM_CONSTANT_Class,
    //         ConstantValue::ConstantFieldref(_, _) => JVM_CONSTANT_Fieldref,
    //         ConstantValue::ConstantMethodref(_, _) => JVM_CONSTANT_Methodref,
    //         ConstantValue::ConstantInterfaceMethodref(_, _) => JVM_CONSTANT_InterfaceMethodref,
    //         ConstantValue::ConstantString(_) => JVM_CONSTANT_String,
    //         ConstantValue::ConstantInteger(_) => JVM_CONSTANT_Integer,
    //         ConstantValue::ConstantFloat(_) => JVM_CONSTANT_Float,
    //         ConstantValue::ConstantLong(_) => JVM_CONSTANT_Long,
    //         ConstantValue::ConstantDouble(_) => JVM_CONSTANT_Double,
    //         ConstantValue::ConstantNameAndType(_, _) => JVM_CONSTANT_NameAndType,
    //         ConstantValue::ConstantUtf8(_) => JVM_CONSTANT_Utf8,
    //         ConstantValue::ConstantMethodHandle(_, _) => JVM_CONSTANT_MethodHandle,
    //         ConstantValue::ConstantMethodType(_) => JVM_CONSTANT_MethodType,
    //         ConstantValue::ConstantDynamic(_, _) => JVM_CONSTANT_Dynamic,
    //         // ConstantValue::ConstantDynamicCallSite => JVM_CONSTANT_InvokeDynamic,
    //         ConstantValue::ConstantInvokeDynamic(_, _) => JVM_CONSTANT_InvokeDynamic,
    //         ConstantValue::ConstantModule(_) => JVM_CONSTANT_Module,
    //         ConstantValue::ConstantPackage(_) => JVM_CONSTANT_Package,
    //         // _ => panic!("未知常量值类型")
    //     };
    //     result as u8
    // }
}

impl ConstantPool {
    pub fn new(capacity: u16) -> ConstantPool {
        let mut pool = ConstantPool {
            count: 0,
            values: Vec::with_capacity(capacity as usize),
            cache: None
        };
        pool.values.push(ConstantItem {
            index: 0,
            value: ConstantValue::Null
        });
        pool
    }
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<ConstantPool> {
        let name = "常量池";
        let pool_count: u16 = reader.read_to(name)?;
        let mut pool = ConstantPool::new(pool_count);
        for _ in 1..pool_count {
            let value = ConstantValue::new_with_reader(reader)?;
            pool.add_constant_force(value);
        }
        Ok(pool)
    }

    pub fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("常量池长度", self.values.len() as u16)?;
        for item in &self.values {
            item.value.write_to(writer)?;
        }
        Ok(())
    }

    fn cache(&mut self) -> &mut HashMap<ConstantValue, u16> {
        match self.cache {
            Some(ref mut cache) => cache,
            None => {
                let mut cache = HashMap::with_capacity(self.count as usize);
                for item in &self.values {
                    cache.insert(item.value.clone(), item.index);
                }
                self.cache = Some(cache);
                self.cache()
            }
        }
    }

    // pub fn add_constant_item(&mut self, item: ConstantItem) -> u16 {
    //     self.count += 1;
    //     self.values.push(item);
    //     self.count
    // }

    pub fn add_constant(&mut self, value: ConstantValue) -> u16 {
        if let Some(index) = self.cache().get(&value) {
            return *index;
        }
        self.add_constant_force(value)
    }

    #[inline]
    fn add_constant_force(&mut self, value: ConstantValue) -> u16 {
        self.count += 1;
        let item = ConstantItem {
            index: self.count,
            value
        };
        if let Some(cache) = &mut self.cache {
            cache.insert(item.value.clone(), self.count);
        }
        self.values.push(item);
        self.count
    }

    pub fn get_constant_item(&self, index: u16) -> &ConstantValue {
        if index >= self.count {
            &self.values[0].value
        } else {
            &self.values[index as usize].value
        }
    }

    pub fn get_constant_count(&self) -> u16 {
        self.count
    }
}

impl Hash for ConstantValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.byte().hash(state);
        match self {
            ConstantValue::Null => {}
            ConstantValue::ConstantClass(v)|
            ConstantValue::ConstantString(v)|
            ConstantValue::ConstantMethodType(v)|
            ConstantValue::ConstantModule(v)|
            ConstantValue::ConstantPackage(v)
                => v.hash(state),
            ConstantValue::ConstantInteger(v) => v.hash(state),
            ConstantValue::ConstantFloat(v) => v.to_bits().hash(state),
            ConstantValue::ConstantLong(v) => v.hash(state),
            ConstantValue::ConstantDouble(v) => v.to_bits().hash(state),
            ConstantValue::ConstantUtf8(v) => v.hash(state),
            ConstantValue::ConstantFieldref(a, b)|
            ConstantValue::ConstantMethodref(a, b)|
            ConstantValue::ConstantInterfaceMethodref(a, b)|
            ConstantValue::ConstantNameAndType(a, b)|
            ConstantValue::ConstantDynamic(a, b)|
            ConstantValue::ConstantInvokeDynamic(a, b)
                => {
                a.hash(state);
                b.hash(state);
            }
            ConstantValue::ConstantMethodHandle(a, b) => {
                a.hash(state);
                b.hash(state);
            }
        }
    }
}

impl Eq for ConstantValue {}

impl PartialEq<Self> for ConstantValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for ConstantValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ConstantValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.byte().cmp(&other.byte());
        if ordering != Ordering::Equal {
            return ordering;
        }
        match (self, other) {
            (ConstantValue::ConstantClass(v1), ConstantValue::ConstantClass(v2))|
            (ConstantValue::ConstantString(v1), ConstantValue::ConstantString(v2))|
            (ConstantValue::ConstantMethodType(v1), ConstantValue::ConstantMethodType(v2))|
            (ConstantValue::ConstantModule(v1), ConstantValue::ConstantModule(v2))|
            (ConstantValue::ConstantPackage(v1), ConstantValue::ConstantPackage(v2))
                => v1.cmp(v2),
            (ConstantValue::ConstantInteger(v1), ConstantValue::ConstantInteger(v2))
                => v1.cmp(v2),
            (ConstantValue::ConstantFloat(v1), ConstantValue::ConstantFloat(v2))
                => v1.total_cmp(v2),
            (ConstantValue::ConstantLong(v1), ConstantValue::ConstantLong(v2))
                => v1.cmp(v2),
            (ConstantValue::ConstantDouble(v1), ConstantValue::ConstantDouble(v2))
                => v1.total_cmp(v2),
            (ConstantValue::ConstantUtf8(v1), ConstantValue::ConstantUtf8(v2))
                => v1.cmp(v2),
            (ConstantValue::ConstantFieldref(a1, b1), ConstantValue::ConstantFieldref(a2, b2))|
            (ConstantValue::ConstantMethodref(a1, b1), ConstantValue::ConstantMethodref(a2, b2))|
            (ConstantValue::ConstantInterfaceMethodref(a1, b1), ConstantValue::ConstantInterfaceMethodref(a2, b2))|
            (ConstantValue::ConstantNameAndType(a1, b1), ConstantValue::ConstantNameAndType(a2, b2))|
            (ConstantValue::ConstantDynamic(a1, b1), ConstantValue::ConstantDynamic(a2, b2))|
            (ConstantValue::ConstantInvokeDynamic(a1, b1), ConstantValue::ConstantInvokeDynamic(a2, b2))
                => {
                let ordering = a1.cmp(a2);
                if ordering != Ordering::Equal {
                    ordering
                } else {
                    b1.cmp(b2)
                }
            }
            (ConstantValue::ConstantMethodHandle(a1, b1), ConstantValue::ConstantMethodHandle(a2, b2))
                => {
                let ordering = a1.cmp(a2);
                if ordering != Ordering::Equal {
                    ordering
                } else {
                    b1.cmp(b2)
                }
            }
            _ => Ordering::Equal,
        }
    }
}