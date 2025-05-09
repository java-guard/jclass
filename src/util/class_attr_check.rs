use crate::classfile_constants::{JVM_CONSTANT_Class, JVM_CONSTANT_Double, JVM_CONSTANT_Dynamic, JVM_CONSTANT_Fieldref, JVM_CONSTANT_Float, JVM_CONSTANT_Integer, JVM_CONSTANT_InterfaceMethodref, JVM_CONSTANT_InvokeDynamic, JVM_CONSTANT_Long, JVM_CONSTANT_MethodHandle, JVM_CONSTANT_MethodType, JVM_CONSTANT_Methodref, JVM_CONSTANT_Module, JVM_CONSTANT_NameAndType, JVM_CONSTANT_Package, JVM_CONSTANT_String, JVM_CONSTANT_Utf8, _bindgen_ty_3};
use crate::common::error::{MessageError, Result};

#[repr(C, align(8))]
#[derive(Debug)]
pub struct DataRange {
    pub start: usize,
    pub end: usize,
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct SimpleClassInfo {
    pub constants_end: usize,
    pub fields_start: usize,
    pub methods_start: usize,
    pub attributes_start: usize,
    pub specify_attribute: Option<DataRange>,
}

#[inline]
pub fn fast_scan_class(data: & [u8], attribute_name: &[u8], not_check_attr: bool) -> Result<Option<SimpleClassInfo>> {
    // magic + minor_version + major_version
    let mut index = 8;
    let constant_size = get_u16_from_data(data, &mut index)?;
    let mut data_key_index = 0;
    let mut name_found = not_check_attr;
    for i in 1..constant_size {
        let is_data_key = get_constant_value_size(data, &mut index, attribute_name, name_found)?;
        if is_data_key {
            name_found = true;
            data_key_index = i;
        }
    }
    if name_found {
        let constants_end = index;
        // access_flags + class_index + superclass_index
        index += 6;
        // interface
        let interface_size = get_u16_from_data(data, &mut index)?;
        index += (interface_size as usize) << 1;
        // field
        let fields_start = index;
        handle_field_or_method(data, &mut index)?;
        // method
        let methods_start = index;
        handle_field_or_method(data, &mut index)?;

        // attribute
        let attributes_start = index;
        let attr_size = get_u16_from_data(data, &mut index)?;
        let mut specify_attribute = None;
        for _ in 0..attr_size {
            // name
            let name_index = get_u16_from_data(data, &mut index)?;
            let data_size = get_u32_from_data(data, &mut index)?;
            let start = index;
            index += data_size as usize;
            if name_index == data_key_index {
                return if index > data.len() {
                    Err(MessageError::new("读取命中的属性内容时越界"))
                } else {
                    specify_attribute = Some(DataRange {
                        start,
                        end: index,
                    });
                    break;
                }
            }
        }
        Ok(Some(SimpleClassInfo {
            constants_end,
            fields_start,
            methods_start,
            attributes_start,
            specify_attribute,
        }))
    } else { 
        Ok(None)
    }
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
pub fn handle_field_or_method(data: &[u8], index: &mut usize) -> Result<()> {
    let size = get_u16_from_data(data, index)?;
    for _ in 0..size {
        // access_flags + name + descriptor
        *index += 6;
        handle_attributes(data, index)?;
    }
    Ok(())
}

#[inline]
fn get_constant_value_size(data: &[u8], index: &mut usize, attribute_name: &[u8], name_found: bool) -> Result<bool> {
    let type_ = match data.get(*index) {
        None => {
            return Err(MessageError::new("读取常量类型时越界"));
        }
        Some(v) => *v
    };
    *index += 1;
    *index += match type_ as _bindgen_ty_3 {
        JVM_CONSTANT_Utf8 => {
            let str_size = get_u16_from_data(data, index)?;
            let str_size = str_size as usize;
            if name_found || str_size != attribute_name.len() {
                *index += str_size;
                return Ok(false);
            }
            let start = *index;
            *index += str_size;
            if *index > data.len() {
                return Err(MessageError::new("读取utf8越界"))
            }

            let eq = &data[start..*index] == attribute_name;
            return Ok(eq);
        }
        JVM_CONSTANT_Integer | JVM_CONSTANT_Float => {
            size_of::<i32>()
        }
        JVM_CONSTANT_Long | JVM_CONSTANT_Double=> {
            size_of::<i64>()
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
        _ => {
            0
        }
    };
    Ok(false)
}

#[inline]
pub fn get_u16_from_data(data: &[u8], index: &mut usize) -> Result<u16> {
    let start = *index;
    *index += 2;
    if *index > data.len() {
        return Err(MessageError::new("读取u16越界"))
    }
    unsafe {
        let ptr = data.as_ptr().add(start) as *const u16;
        Ok(u16::from_be(ptr.read_unaligned()))
    }
}

#[inline]
pub fn get_u32_from_data(data: &[u8], index: &mut usize) -> Result<u32> {
    let start = *index;
    *index += 4;
    if *index > data.len() {
        return Err(MessageError::new("读取u32越界"))
    }
    unsafe {
        let ptr = data.as_ptr().add(start) as *const u32;
        Ok(u32::from_be(ptr.read_unaligned()))
    }
}