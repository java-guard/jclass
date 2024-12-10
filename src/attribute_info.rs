use crate::common::Reader;
use crate::constant_pool::ConstantPool;
use crate::util::reader_utils::{read_bytes, ReadToType};

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
    pub fn new_from_reader(reader: &mut Reader, pool: &ConstantPool) -> crate::common::Result<AttributeInfo> {
        let name_index: u16 = reader.read_to("属性")?;
        // todo
        Ok(AttributeInfo::SimpleAttribute(SimpleAttribute::new_from_reader(reader, name_index)?))
    }
}

impl SimpleAttribute {
    pub fn new_from_reader(reader: &mut Reader, name: u16) -> crate::common::Result<SimpleAttribute> {
        let len: u32 = reader.read_to("属性数据长度")?;
        let mut data = vec![0; len as usize];
        read_bytes(reader, "属性数据", &mut data)?;
        Ok(SimpleAttribute {
            name,
            data
        })
    }
}