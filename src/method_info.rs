use crate::attribute_info::OriginAttribute;
use crate::support::data_reader::{DataReader, DataWriter, ReadToType, WriteFromType};
use crate::common::error::Result;
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name: u16,
    pub descriptor: u16,
    pub attributes: Vec<OriginAttribute>
}

impl MethodInfo {
    pub fn new_from_reader<T: Read>(reader: &mut DataReader<T>) -> Result<MethodInfo> {
        let access_flags: u16 = reader.read_to("方法访问标识")?;
        let name: u16 = reader.read_to("方法名")?;
        let descriptor: u16 = reader.read_to("方法描述")?;
        let attribute_size: u16 = reader.read_to("方法属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_size as usize);
        for _ in 0..attribute_size {
            attributes.push(OriginAttribute::new_from_reader(reader)?)
        }
        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes
        })
    }

    pub fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("方法访问标识", self.access_flags)?;
        writer.write_from("方法名", self.name)?;
        writer.write_from("方法描述", self.descriptor)?;
        writer.write_from("方法属性数量", self.attributes.len() as u16)?;
        for attribute in &self.attributes {
            attribute.write_to(writer)?;
        }
        Ok(())
    }
}