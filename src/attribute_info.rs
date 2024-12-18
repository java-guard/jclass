use crate::constant_pool::ConstantPool;
use crate::lazy_value::LazyValue;
use crate::support::data_reader::DataReader;
use crate::support::data_reader::ReadToType;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum AttributeInfo {
    AnnotationDefaultAttribute(AnnotationDefaultAttribute),
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
    OriginAttribute,
}

#[derive(Clone, Debug)]
pub struct OriginAttribute {
    name: u16,
    data: Vec<u8>,
    info: LazyValue<AttributeInfo>,
}

// trait ParsedAttribute {
//     fn parse(name: u16, data: &[u8], pool: &ConstantPool) -> Self;
//     fn name(&self) -> &str;
//     fn data(&self) -> Vec<u8>;
// }

// pub struct DefaultAttribute {
//     name: String,
//     data: Vec<u8>
// }

#[derive(Clone, Debug)]
pub struct AnnotationDefaultAttribute {
    name: String,

}

impl OriginAttribute {
    pub fn new_from_reader<T: Read>(reader: &mut DataReader<T>) -> crate::error::Result<OriginAttribute> {
        let name_index: u16 = reader.read_to("属性")?;
        let len: u32 = reader.read_to("属性数据长度")?;
        let mut data = vec![0; len as usize];
        reader.read_bytes("属性数据", &mut data)?;
        Ok(OriginAttribute {
            name: name_index,
            data,
            info: LazyValue::UnLoad
        })
    }
}