use std::io::Read;
use crate::util::io_utils::{read_class_bytes, read_class_bytes_u32};

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

pub struct SimpleAttribute {
    name: u16,
    data: Vec<u8>
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