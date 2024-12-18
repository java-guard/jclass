use crate::attribute_info::OriginAttribute;
use crate::support::data_reader::{DataReader, ReadToType};
use std::io::Read;

#[derive(Clone, Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attributes: Vec<OriginAttribute>
}

impl FieldInfo {
    pub fn new_from_reader<T: Read>(reader: &mut DataReader<T>) -> crate::error::Result<FieldInfo> {
        let access_flags: u16 = reader.read_to("字段访问标识")?;
        let name: u16 = reader.read_to("字段名")?;
        let descriptor: u16 = reader.read_to("字段描述")?;
        let attribute_size: u16 = reader.read_to("字段属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_size as usize);
        for _ in 0..attribute_size {
            attributes.push(OriginAttribute::new_from_reader(reader)?)
        }
        Ok(FieldInfo {
            access_flags,
            name,
            descriptor,
            attributes
        })
    }
}