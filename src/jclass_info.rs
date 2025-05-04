use std::io::{Read, Write};
use crate::attribute_info::OriginAttribute;
use crate::common::error::{Result, MessageError};
use crate::constant_pool::ConstantPool;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;
use crate::support::data_reader::{DataReader, DataWriter, ReadToType, WriteFromType};

pub const JCLASS_MAGIC: u32 = 0xCAFEBABE;

#[derive(Debug, Clone, Default)]
pub struct JClassInfo {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub class_index: u16,
    pub superclass_index: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<OriginAttribute>,
}

impl JClassInfo {
    pub fn from_reader<T: Read>(reader: &mut DataReader<T>) -> Result<JClassInfo> {
        let magic: u32 = reader.read_to("魔术头")?;
        if magic != JCLASS_MAGIC {
            return Err(MessageError::new("解析数据非class文件"));
        }
        let minor_version: u16 = reader.read_to("次版本")?;
        let major_version: u16 = reader.read_to("主版本")?;
        let constant_pool = ConstantPool::new_with_reader(reader)?;
        let access_flags: u16 = reader.read_to("访问标志")?;
        let class_index: u16 = reader.read_to("该类索引")?;
        let superclass_index: u16 = reader.read_to("父类索引")?;
        let interface_count: u16 = reader.read_to("接口数量")?;
        let mut interfaces = Vec::with_capacity(interface_count as usize);
        for _ in 0..interface_count {
            let interface: u16 = reader.read_to("接口索引")?;
            interfaces.push(interface);
        }

        let field_count: u16 = reader.read_to("字段数量")?;
        let mut fields = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            let field_info = FieldInfo::new_from_reader(reader)?;
            fields.push(field_info);
        }

        let method_count: u16 = reader.read_to("方法数量")?;
        let mut methods = Vec::with_capacity(method_count as usize);
        for _ in 0..method_count {
            let method_info = MethodInfo::new_from_reader(reader)?;
            methods.push(method_info);
        }

        let attribute_count: u16 = reader.read_to("属性数量")?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            let attribute = OriginAttribute::new_from_reader(reader)?;
            attributes.push(attribute);
        }
        Ok(JClassInfo {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            class_index,
            superclass_index,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }


    pub fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("魔术头", JCLASS_MAGIC)?;
        writer.write_from("次版本", self.minor_version)?;
        writer.write_from("主版本", self.major_version)?;
        self.constant_pool.write_to(writer)?;
        writer.write_from("访问标志", self.access_flags)?;
        writer.write_from("该类索引", self.class_index)?;
        writer.write_from("父类索引", self.superclass_index)?;
        writer.write_from("接口数量", self.interfaces.len() as u16)?;
        for interface in &self.interfaces {
            writer.write_from("接口索引", *interface)?;
        }

        writer.write_from("字段数量", self.fields.len() as u16)?;
        for field in &self.fields {
            field.write_to(writer)?;
        }

        writer.write_from("方法数量", self.methods.len() as u16)?;
        for method in &self.methods {
            method.write_to(writer)?;
        }

        writer.write_from("属性数量", self.attributes.len() as u16)?;
        for attribute in &self.attributes {
            attribute.write_to(writer)?;
        }
        Ok(())
    }
}