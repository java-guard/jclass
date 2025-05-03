use std::io::Read;
use crate::attribute_info::OriginAttribute;
use crate::constant_pool::ConstantPool;
use crate::error::MessageError;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;
use crate::support::data_reader::{DataReader, ReadToType};

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
    pub fn from_reader<T: Read>(reader: &mut DataReader<T>) -> crate::error::Result<JClassInfo> {
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
}