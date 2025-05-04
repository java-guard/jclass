use crate::common::error::Result;
use crate::support::data_reader::DataReader;
use crate::support::data_reader::ReadToType;
use std::io::{Cursor, Read};

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
        let name_index: u16 = reader.read_to("属性")?;
        let len: u32 = reader.read_to("属性数据长度")?;
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
}

// 专为JGLauncher提供，仅需用到CodeAttribute，所以咱不实现所有Attribute的解析
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