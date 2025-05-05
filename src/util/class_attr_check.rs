use crate::classfile_constants::{JVM_CONSTANT_Class,
                                 JVM_CONSTANT_Double,
                                 JVM_CONSTANT_Dynamic,
                                 JVM_CONSTANT_Fieldref,
                                 JVM_CONSTANT_Float,
                                 JVM_CONSTANT_Integer,
                                 JVM_CONSTANT_InterfaceMethodref,
                                 JVM_CONSTANT_InvokeDynamic,
                                 JVM_CONSTANT_Long,
                                 JVM_CONSTANT_MethodHandle,
                                 JVM_CONSTANT_MethodType,
                                 JVM_CONSTANT_Methodref,
                                 JVM_CONSTANT_Module,
                                 JVM_CONSTANT_NameAndType,
                                 JVM_CONSTANT_Package,
                                 JVM_CONSTANT_String,
                                 JVM_CONSTANT_Utf8};
use crate::common::error::{Result, MessageError};

pub fn check_class_has_attribute<'a>(data: &'a [u8], attribute_name: &[u8]) -> Result<Option<&'a [u8]>> {
    // magic + minor_version + major_version
    let mut index = 8;
    let constant_size = get_u16_from_data(data, &mut index)?;
    let mut data_key_index = 0;
    for i in 1..constant_size {
        let is_data_key = get_constant_value_size(data, &mut index, attribute_name)?;
        if is_data_key {
            data_key_index = i;
        }
    }
    if data_key_index == 0 {
        return Ok(None);
    }
    // access_flags + class_index + superclass_index
    index += 6;
    // interface
    let interface_size = get_u16_from_data(data, &mut index)?;
    index += (interface_size as usize) << 1;
    // field
    handle_field_or_method(data, &mut index)?;
    // method
    handle_field_or_method(data, &mut index)?;

    // attribute
    let attr_size = get_u16_from_data(data, &mut index)?;
    for _ in 0..attr_size {
        // name
        let name_index = get_u16_from_data(data, &mut index)?;
        let data_size = get_u32_from_data(data, &mut index)?;
        if name_index == data_key_index {
            return Ok(Some(&data[index..index+data_size as usize]));
        }

        index += data_size as usize;
    }
    Ok(None)
}

#[inline]
fn handle_attributes(data: &[u8], index: &mut usize) -> Result<()> {
    let attr_size = get_u16_from_data(data, index)?;
    for _ in 0..attr_size {
        // name
        *index += 2;
        let data_size = get_u32_from_data(data, index)?;
        *index += data_size as usize;
    }
    Ok(())
}

#[inline]
fn handle_field_or_method(data: &[u8], index: &mut usize) -> Result<()> {
    let size = get_u16_from_data(data, index)?;
    for _ in 0..size {
        // access_flags + name + descriptor
        *index += 6;
        handle_attributes(data, index)?;
    }
    Ok(())
}

#[inline]
fn get_constant_value_size(data: &[u8], index: &mut usize, attribute_name: &[u8]) -> Result<bool> {
    let type_ = data[*index];
    *index += 1;
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
            size_of::<i32>()
        }
        JVM_CONSTANT_Float => {
            size_of::<f32>()
        }
        JVM_CONSTANT_Long => {
            size_of::<i64>()
        }
        JVM_CONSTANT_Double => {
            size_of::<f64>()
        }
        JVM_CONSTANT_Utf8 => {
            let str_size = get_u16_from_data(data, index)?;
            let str_size = str_size as usize;
            if *index > data.len() - str_size {
                return Err(MessageError::new("读取utf8越界"))
            }
            let eq = str_size == attribute_name.len() && &data[*index..*index + str_size] == attribute_name;
            *index += str_size;
            return Ok(eq);
        }
        _ => {
            0
        }
    };
    Ok(false)
}

#[inline]
fn get_u16_from_data(data: &[u8], index: &mut usize) -> Result<u16> {
    if *index > data.len() - 2 {
        return Err(MessageError::new("读取u16越界"))
    }
    unsafe {
        let ptr = data.as_ptr().add(*index) as *const u16;
        *index += 2;
        Ok((*ptr).swap_bytes())
    }
}

#[inline]
fn get_u32_from_data(data: &[u8], index: &mut usize) -> Result<u32> {
    if *index > data.len() - 4 {
        return Err(MessageError::new("读取u32越界"))
    }
    unsafe {
        let ptr = data.as_ptr().add(*index) as *const u32;
        *index += 4;
        Ok((*ptr).swap_bytes())
    }
}