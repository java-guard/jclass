use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8};
use crate::error::{MessageError, Result, ToResult};
use crate::support::data_reader::{DataReader, ReadToType};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Read;

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

#[derive(Debug, Clone)]
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
        let mut single_byte = [0;1];
        let len = reader.read(&mut single_byte).with_message("常量类型读取出错")?;
        if len != 1 {
            return Err(MessageError::new("常量类型无法读取"));
        }
        Ok(match single_byte[0].into() {
            JVM_CONSTANT_Class => {
                ConstantValue::ConstantClass(reader.read_to("Class常量")?)
            }
            JVM_CONSTANT_Fieldref => {
                ConstantValue::ConstantFieldref(reader.read_to("ConstantField ref: class index 值读取失败")?,
                                                reader.read_to("ConstantField ref: name index 值读取失败")?)
            }
            JVM_CONSTANT_Methodref => {
                ConstantValue::ConstantMethodref(reader.read_to("ConstantField ref: class index 值读取失败")?,
                                                reader.read_to("ConstantField ref: name index 值读取失败")?)
            }
            JVM_CONSTANT_InterfaceMethodref => {
                ConstantValue::ConstantInterfaceMethodref(reader.read_to("ConstantField ref: class index 值读取失败")?,
                                                reader.read_to("ConstantField ref: name index 值读取失败")?)
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
                let str_bytes = reader.read_bytes_with_pre_size("UTF8字符串常量")?;
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
    pub fn new(size: u16) -> ConstantPool {
        let mut pool = ConstantPool {
            count: 0,
            values: Vec::with_capacity(size as usize),
            cache: None
        };
        pool.values.push(ConstantItem {
            index: 0,
            value: ConstantValue::Null
        });
        pool
    }
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<ConstantPool> {
        // let now = Instant::now();
        let name = "常量池";
        let pool_count: u16 = reader.read_to(name)?;
        let mut pool = ConstantPool::new(pool_count);
        for _ in 1..pool_count {
            // let now = Instant::now();
            let value = ConstantValue::new_with_reader(reader)?;
            // println!(">>>> pool item: {:?}: {:?}", now.elapsed(), &value);
            pool.add_constant_force(&value);
        }
        // println!(">>> pool: {:?}", now.elapsed());
        Ok(pool)
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

    pub fn add_constant(&mut self, value: &ConstantValue) -> u16 {
        if let Some(index) = self.cache().get(value) {
            return *index;
        }
        self.add_constant_force(value)
    }

    fn add_constant_force(&mut self, value: &ConstantValue) -> u16 {
        self.count += 1;
        self.values.push(ConstantItem {
            index: self.count,
            value: value.clone()
        });
        if let Some(cache) = &mut self.cache {
            cache.insert(value.clone(), self.count);
        }
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