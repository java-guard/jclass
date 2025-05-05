use crate::common::error::Result;
use crate::support::data_reader::{DataReader, DataWriter, WriteFromType};
use crate::support::data_reader::ReadToType;
use std::io::{BufWriter, Cursor, Read, Write};

#[derive(Clone, Debug)]
pub struct OriginAttribute {
    pub name: u16,
    pub data: Vec<u8>,
    // 专为JGLauncher提供，仅需用到CodeAttribute，所以咱不实现所有Attribute的解析
    // info: LazyValue<AttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct CodeAttribute {
    pub codes: Vec<u8>,
    pub max_stack: u16,
    pub max_locals: u16,
    pub exceptions: ExceptionTable,
    pub attributes: Vec<OriginAttribute>,
}

#[derive(Clone, Debug)]
pub struct ExceptionTable {
    pub entries: Vec<ExceptionTableEntry>,
}

#[derive(Clone, Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl OriginAttribute {
    pub fn new_from_reader<T: Read>(reader: &mut DataReader<T>) -> Result<OriginAttribute> {
        let name_index: u16 = reader.read_to("属性名")?;
        let len: i32 = reader.read_to("属性数据长度")?;
        let len = len as usize;
        let mut data = Vec::with_capacity(len);
        unsafe {
            data.set_len(len);
        }
        reader.read_bytes("属性数据", &mut data)?;
        Ok(OriginAttribute {
            name: name_index,
            data,
            // info: LazyValue::UnLoad
        })
    }

    pub fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("属性名", self.name)?;
        writer.write_from("属性数据长度", self.data.len() as i32)?;
        writer.write_bytes("属性数据", &self.data)
    }

    #[inline]
    pub fn byte_size(&self) -> usize {
        size_of::<u16>() + size_of::<i32>() // data长度
            + self.data.len()
    }
}

impl CodeAttribute {
    pub fn new_with_data(data: &[u8]) -> Result<CodeAttribute> {
        Self::new_with_reader(&mut Cursor::new(data).into())
    }

    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<CodeAttribute> {
        // let _attr_len: i32 = reader.read_to("属性长度")?;
        let max_stack: u16 = reader.read_to("操作栈最大深度")?;
        let max_locals: u16 = reader.read_to("局部变量最大槽数")?;

        let code_len: i32 = reader.read_to("字节码长度")?;
        let code_len = code_len as usize;
        let mut codes = Vec::with_capacity(code_len);
        unsafe {
            codes.set_len(code_len);
        }
        reader.read_bytes("字节码", &mut codes)?;

        let exceptions = ExceptionTable::new_with_reader(reader)?;

        let attr_size: u16 = reader.read_to("属性数量")?;
        let mut attributes = Vec::with_capacity(attr_size as usize);
        for i in 0..attr_size {
            attributes.push(OriginAttribute::new_from_reader(reader)?);
        }

        Ok(CodeAttribute {
            codes,
            max_stack,
            max_locals,
            exceptions,
            attributes,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let data_size = self.byte_size();
        let mut data = Vec::with_capacity(data_size);
        {
            let mut writer = DataWriter::from(BufWriter::new(&mut data));
            writer.write_from("操作栈最大深度", self.max_stack)?;
            writer.write_from("局部变量最大槽数", self.max_locals)?;
            writer.write_from("字节码长度", self.codes.len() as i32)?;
            writer.write_bytes("字节码", &self.codes)?;
            self.exceptions.write_to(&mut writer)?;
            writer.write_from("属性数量", self.attributes.len() as u16)?;
            for attr in &self.attributes {
                attr.write_to(&mut writer)?;
            }
        }
        Ok(data)
    }

    pub fn byte_size(&self) -> usize {
        let mut attrs_size = 0;
        for attr in &self.attributes {
            attrs_size += attr.byte_size();
        }
        self.codes.len() + size_of::<[u16;2]>() + self.exceptions.byte_size() + attrs_size
    }
}

impl ExceptionTable {
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<ExceptionTable> {
        let count: u16 = reader.read_to("异常表大小")?;
        let mut entries = Vec::with_capacity(count as usize);
        for i in 0..count {
            entries.push(ExceptionTableEntry::new_with_reader(reader)?);
        }
        Ok(ExceptionTable {
            entries,
        })
    }

    #[inline]
    fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("异常表大小", self.entries.len() as u16)?;
        for entry in &self.entries {
            entry.write_to(writer)?;
        }
        Ok(())
    }

    #[inline]
    pub fn byte_size(&self) -> usize {
        self.entries.len() * ExceptionTableEntry::byte_size()
    }
}

impl ExceptionTableEntry {
    pub fn new_with_reader<T: Read>(reader: &mut DataReader<T>) -> Result<ExceptionTableEntry> {
        let start_pc = reader.read_to("起始PC")?;
        let end_pc = reader.read_to("结束PC")?;
        let handler_pc = reader.read_to("跳转PC")?;
        let catch_type = reader.read_to("捕获类型")?;
        Ok(ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }

    #[inline]
    fn write_to<T: Write>(&self, writer: &mut DataWriter<T>) -> Result<()> {
        writer.write_from("起始PC", self.start_pc)?;
        writer.write_from("结束PC", self.end_pc)?;
        writer.write_from("跳转PC", self.handler_pc)?;
        writer.write_from("捕获类型", self.catch_type)
    }

    #[inline]
    pub fn byte_size() -> usize {
        size_of::<[u16;4]>()
    }
}

// 专为JGLauncher提供，仅需用到CodeAttribute，所以暂不实现所有Attribute的解析
// #[derive(Clone, Debug)]
// pub enum AttributeInfo {
//     AnnotationDefaultAttribute,
//     BootstrapMethodsAttribute,
//     CodeAttribute,
//     ConstantAttribute,
//     DeprecatedAttribute,
//     EnclosingMethodAttribute,
//     ExceptionsAttribute,
//     InnerClassesAttribute,
//     LineNumberAttribute,
//     LocalVariableAttribute,
//     LocalVariableTypeAttribute,
//     MethodParametersAttribute,
//     NestHostAttribute,
//     NestMembersAttribute,
//     AnnotationsAttribute,
//     ParameterAnnotationsAttribute,
//     TypeAnnotationsAttribute,
//     SignatureAttribute,
//     SourceFileAttribute,
//     SyntheticAttribute,
//     StackMap,
//     StackMapTable,
//     OriginAttribute,
// }