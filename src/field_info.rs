use crate::attribute_info::{AttributeInfo, SimpleAttribute};
use crate::common::Reader;
use crate::util::reader_utils::ReadToType;

#[derive(Clone, Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attributes: Vec<AttributeInfo>
}

impl FieldInfo {
    pub fn new_from_reader(reader: &mut Reader) -> crate::common::Result<FieldInfo> {
        let access_flags: u16 = reader.read_to("字段访问标识")?;
        let name: u16 = reader.read_to("字段名")?;
        let descriptor: u16 = reader.read_to("字段描述")?;
        let attribute_size: u16 = reader.read_to("字段属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_size as usize);
        for _ in 0..attribute_size {
            let attribute_name: u16 = reader.read_to("字段属性名")?;
            attributes.push(AttributeInfo::SimpleAttribute(SimpleAttribute::new_from_reader(reader, attribute_name)?))
        }
        Ok(FieldInfo {
            access_flags,
            name,
            descriptor,
            attributes
        })
    }
}