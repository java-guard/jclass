use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8};

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

impl ConstantValue {
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