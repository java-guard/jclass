use std::io::Read;
use crate::constant_pool::ConstantPool;
use crate::util::io_utils::{read_class_bytes, read_class_bytes_u16, read_class_bytes_u32};

#[derive(Clone, Debug)]
pub enum AttributeInfo {
    AnnotationDefaultAttribute,
    BootstrapMethodsAttribute,
    CodeAttribute,
    ConstantAttribute,
    DeprecatedAttribute,
    EnclosingMethodAttribute,
    ExceptionsAttribute,
    InnerClassesAttribute,
    LineNumberAttribute,
    LocalVariableAttribute,
    LocalVariableTypeAttribute,
    MethodParametersAttribute,
    NestHostAttribute,
    NestMembersAttribute,
    AnnotationsAttribute,
    ParameterAnnotationsAttribute,
    TypeAnnotationsAttribute,
    SignatureAttribute,
    SourceFileAttribute,
    SyntheticAttribute,
    StackMap,
    StackMapTable,
    SimpleAttribute(SimpleAttribute),
}

#[derive(Clone, Debug)]
pub struct SimpleAttribute {
    name: u16,
    data: Vec<u8>
}

impl AttributeInfo {
    pub fn new_from_reader<T: Read>(reader: &mut T, pool: &ConstantPool) -> crate::common::Result<AttributeInfo> {
        let name_index = read_class_bytes_u16(reader, "属性")?;
        // todo
        Ok(AttributeInfo::SimpleAttribute(SimpleAttribute::new_from_reader(reader, name_index)?))
    }
}

impl SimpleAttribute {
    pub fn new_from_reader<T: Read>(reader: &mut T, name: u16) -> crate::common::Result<SimpleAttribute> {
        let len = read_class_bytes_u32(reader, "属性数据长度")?;
        let data = read_class_bytes(reader, "属性数据", len as usize)?;
        Ok(SimpleAttribute {
            name,
            data
        })
    }
}