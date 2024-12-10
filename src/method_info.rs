use crate::attribute_info::{AttributeInfo, SimpleAttribute};
use crate::util::io_utils::read_class_bytes_u16;
use std::io::Read;

#[derive(Clone, Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attributes: Vec<AttributeInfo>
}

impl MethodInfo {
    pub fn new_from_reader<T: Read>(reader: &mut T) -> crate::common::Result<MethodInfo> {
        let access_flags = read_class_bytes_u16(reader, "方法访问标识")?;
        let name = read_class_bytes_u16(reader, "方法名")?;
        let descriptor = read_class_bytes_u16(reader, "方法描述")?;
        let attribute_size = read_class_bytes_u16(reader, "方法属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_size as usize);
        for _ in 0..attribute_size {
            let attribute_name = read_class_bytes_u16(reader, "方法属性名")?;
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