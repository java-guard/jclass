use crate::common::MessageError;
use crate::constant_pool::ConstantValue;
use std::mem;

#[derive(Clone, Debug)]
pub enum LazyValue<T: Clone> {
    UnLoad,
    None,
    Err(MessageError),
    Some(T),
}

impl<T: Clone> Default for LazyValue<T> {
    fn default() -> Self {
        LazyValue::UnLoad
    }
}

impl<T: Clone> LazyValue<T> {
    pub fn none(&mut self) -> LazyValue<T> {
        mem::replace(self, LazyValue::None)
    }
    pub fn err(&mut self, e: MessageError) -> LazyValue<T> {
        mem::replace(self, LazyValue::Err(e))
    }
    pub fn some(&mut self, value: T) -> LazyValue<T> {
        mem::replace(self, LazyValue::Some(value))
    }
    pub fn update(&mut self, value: LazyValue<T>) -> LazyValue<T> {
        mem::replace(self, value)
    }
    pub fn to_option(&self) -> Option<T> {
        match self {
            LazyValue::Some(v) => Some(v.clone()),
            _ => None
        }
    }

    pub fn to_result(&self, name: &str) -> crate::common::Result<T> {
        match self {
            LazyValue::Err(e) => Err(e.clone()),
            LazyValue::Some(v) => Ok(v.clone()),
            _ => MessageError::new(&format!("[{name}]值为空")).into(),
        }
    }

    pub fn is_load(&self) -> bool {
        match self {
            LazyValue::UnLoad => false,
            _ => true
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JClassInfo {
    pub (crate) magic: LazyValue<[u8;4]>,
    pub (crate) minor_version: LazyValue<u16>,
    pub (crate) major_version: LazyValue<u16>,
    pub (crate) constant_value: LazyValue<ConstantValue>,
    pub (crate) access_flags: LazyValue<u16>,
    pub (crate) class_index: LazyValue<u16>,
    pub (crate) superclass_index: LazyValue<u16>,
    pub (crate) interfaces_count: LazyValue<u16>,
    pub (crate) interfaces_index: LazyValue<u16>,
    pub (crate) fields_count: LazyValue<u16>,
    pub (crate) fields_table: LazyValue<u64>,
    pub (crate) methods_count: LazyValue<u16>,
    pub (crate) methods_table: LazyValue<u64>,
    pub (crate) attributes_count: LazyValue<u16>,
    pub (crate) attributes_table: LazyValue<u64>,
}

impl JClassInfo {
    pub fn magic(&self) -> Option<[u8;4]> {
        self.magic.to_option()
    }
    pub fn set_magic(&mut self, value: [u8;4]) {
        self.magic.some(value);
    }
    pub fn minor_version(&self) -> Option<u16> {
        self.minor_version.to_option()
    }
    pub fn set_minor_version(&mut self, value: u16) {
        self.minor_version.some(value);
    }
    pub fn major_version(&self) -> Option<u16> {
        self.major_version.to_option()
    }
    pub fn set_major_version(&mut self, value: u16) {
        self.major_version.some(value);
    }
    pub fn constant_value(&self) -> Option<ConstantValue> {
        self.constant_value.to_option()
    }
    pub fn set_constant_value(&mut self, value: ConstantValue) {
        self.constant_value.some(value);
    }
    pub fn access_flags(&self) -> Option<u16> {
        self.access_flags.to_option()
    }
    pub fn set_access_flags(&mut self, value: u16) {
        self.access_flags.some(value);
    }
    pub fn class_index(&self) -> Option<u16> {
        self.class_index.to_option()
    }
    pub fn set_class_index(&mut self, value: u16) {
        self.class_index.some(value);
    }
    pub fn superclass_index(&self) -> Option<u16> {
        self.superclass_index.to_option()
    }
    pub fn set_superclass_index(&mut self, value: u16) {
        self.superclass_index.some(value);
    }
    pub fn interfaces_count(&self) -> Option<u16> {
        self.interfaces_count.to_option()
    }
    pub fn set_interfaces_count(&mut self, value: u16) {
        self.interfaces_count.some(value);
    }
    pub fn interfaces_index(&self) -> Option<u16> {
        self.interfaces_index.to_option()
    }
    pub fn set_interfaces_index(&mut self, value: u16) {
        self.interfaces_index.some(value);
    }
    pub fn fields_count(&self) -> Option<u16> {
        self.fields_count.to_option()
    }
    pub fn set_fields_count(&mut self, value: u16) {
        self.fields_count.some(value);
    }
    pub fn fields_table(&self) -> Option<u64> {
        self.fields_table.to_option()
    }
    pub fn set_fields_table(&mut self, value: u64) {
        self.fields_table.some(value);
    }
    pub fn methods_count(&self) -> Option<u16> {
        self.methods_count.to_option()
    }
    pub fn set_methods_count(&mut self, value: u16) {
        self.methods_count.some(value);
    }
    pub fn methods_table(&self) -> Option<u64> {
        self.methods_table.to_option()
    }
    pub fn set_methods_table(&mut self, value: u64) {
        self.methods_table.some(value);
    }
    pub fn attributes_count(&self) -> Option<u16> {
        self.attributes_count.to_option()
    }
    pub fn set_attributes_count(&mut self, value: u16) {
        self.attributes_count.some(value);
    }
    pub fn attributes_table(&self) -> Option<u64> {
        self.attributes_table.to_option()
    }
    pub fn set_attributes_table(&mut self, value: u64) {
        self.attributes_table.some(value);
    }
}