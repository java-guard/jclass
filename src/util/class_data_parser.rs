use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Utf8,
                                  JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic,
                                  JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref,
                                  JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String };
use crate::common::error::{Result, MessageError};
use crate::util::class_attr_check::{get_u16_from_data, get_u32_from_data, handle_field_or_method};

#[derive(Debug)]
pub struct DataRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct ConstCodeInfo {
    pub constants: Vec<DataRange>,
    pub constants_index: Vec<usize>,
    pub codes: Vec<DataRange>,
}

const CODE_ATTR_NAME: &[u8] = "Code".as_bytes();

pub fn class_const_code_parse(data: & [u8]) -> Result<ConstCodeInfo> {
    // magic + minor_version + major_version
    let mut index = 8;
    let constant_size = get_u16_from_data(data, &mut index)?;
    let mut constants = Vec::with_capacity(constant_size as usize >> 1);
    let mut constants_index = vec![0; constant_size as usize];
    let mut data_key_index = 0;
    for i in 1..constant_size {
        let start = index;
        let (is_data_key, is_data_constant) = get_constant_value_size(data, &mut index, CODE_ATTR_NAME)?;
        if is_data_constant {
            // 枚举为 int, float, long, double, uft8 时
            constants_index[i as usize] = constants.len();
            constants.push(DataRange{
                start: start + 1,
                end: index,
            });
        }
        if is_data_key {
            data_key_index = i;
        }
    }
    if data_key_index == 0 {
        return Err(MessageError::new("class中没有Code字符串"));
    }
    // access_flags + class_index + superclass_index
    index += 6;
    // interface
    let interface_size = get_u16_from_data(data, &mut index)?;
    index += (interface_size as usize) << 1;
    // field
    handle_field_or_method(data, &mut index)?;
    // method
    // handle_field(data, &mut index)?;
    let size = get_u16_from_data(data, &mut index)?;
    let mut codes = Vec::with_capacity(size as usize);
    for _ in 0..size {
        // access_flags + name + descriptor
        index += 6;
        let attr_size = get_u16_from_data(data, &mut index)?;
        for _ in 0..attr_size {
            // name
            let name_index = get_u16_from_data(data, &mut index)?;
            let data_size = get_u32_from_data(data, &mut index)?;
            let data_size = data_size as usize;
            let start = index;
            index += data_size;
            if name_index == data_key_index {
                if index > data.len() {
                    return Err(MessageError::new("读取命中的属性内容时越界"))
                } else {
                    codes.push(DataRange {
                        start,
                        end: index,
                    })
                }
            }
        }
    }

    // attribute ignore
    Ok(ConstCodeInfo {
        constants,
        constants_index,
        codes,
    })
}

#[inline]
fn get_constant_value_size(data: &[u8], index: &mut usize, attribute_name: &[u8]) -> Result<(bool, bool)> {
    if *index >= data.len() {
        return Err(MessageError::new("读取常量类型时越界"));
    }
    let type_ = data[*index];
    *index += 1;
    let mut data_constant = false;
    *index += match type_.into() {
        0 => {
            0
        }
        JVM_CONSTANT_Class |
        JVM_CONSTANT_String | JVM_CONSTANT_Module |
        JVM_CONSTANT_Package | JVM_CONSTANT_MethodType => {
            size_of::<u16>()
        }
        JVM_CONSTANT_Fieldref | JVM_CONSTANT_Methodref |
        JVM_CONSTANT_InterfaceMethodref | JVM_CONSTANT_NameAndType |
        JVM_CONSTANT_Dynamic | JVM_CONSTANT_InvokeDynamic => {
            size_of::<[u16;2]>()
        }
        JVM_CONSTANT_MethodHandle => {
            size_of::<u16>() + size_of::<u8>()
        }
        JVM_CONSTANT_Integer => {
            data_constant = true;
            size_of::<i32>()
        }
        JVM_CONSTANT_Float => {
            data_constant = true;
            size_of::<f32>()
        }
        JVM_CONSTANT_Long => {
            data_constant = true;
            size_of::<i64>()
        }
        JVM_CONSTANT_Double => {
            data_constant = true;
            size_of::<f64>()
        }
        JVM_CONSTANT_Utf8 => {
            let str_size = get_u16_from_data(data, index)?;
            let str_size = str_size as usize;
            let start = *index;
            *index += str_size;
            if *index > data.len() {
                return Err(MessageError::new("读取utf8越界"))
            }
            let eq = str_size == attribute_name.len() && &data[start..*index] == attribute_name;
            return Ok((eq, true));
        }
        _ => {
            0
        }
    };
    Ok((false, data_constant))
}