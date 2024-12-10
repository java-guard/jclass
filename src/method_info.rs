use crate::attribute_info::{AttributeInfo, SimpleAttribute};
use crate::common::Reader;
use crate::util::reader_utils::ReadToType;

#[derive(Clone, Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attributes: Vec<AttributeInfo>
}

impl MethodInfo {
    pub fn new_from_reader(reader: &mut Reader) -> crate::common::Result<MethodInfo> {
        let access_flags: u16 = reader.read_to("方法访问标识")?;
        let name: u16 = reader.read_to("方法名")?;
        let descriptor: u16 = reader.read_to("方法描述")?;
        let attribute_size: u16 = reader.read_to("方法属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_size as usize);
        for _ in 0..attribute_size {
            let attribute_name: u16 = reader.read_to("方法属性名")?;
            attributes.push(AttributeInfo::SimpleAttribute(SimpleAttribute::new_from_reader(reader, attribute_name)?))
        }
        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes
        })
    }
}