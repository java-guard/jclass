use crate::attribute_info::AttributeInfo;

pub struct FieldInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attribute: Vec<AttributeInfo>
}