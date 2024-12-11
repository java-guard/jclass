use crate::attribute_info::AttributeInfo;
use crate::error::MessageError;
use crate::constant_pool::ConstantPool;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;
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
    pub fn get(&self) -> Option<T> {
        match self {
            LazyValue::Some(v) => Some(v.clone()),
            _ => None
        }
    }
    pub fn get_ref(&self) -> Option<&T> {
        match &self {
            LazyValue::Some(v) => Some(v),
            _ => None
        }
    }
    pub fn get_mut_ref(&mut self) -> Option<&mut T> {
        match self {
            LazyValue::Some(v) => Some(v),
            _ => None
        }
    }
    pub fn to_option_with_err(&self) -> crate::error::Result<Option<T>> {
        match self {
            LazyValue::Some(v) => Ok(Some(v.clone())),
            LazyValue::Err(e) => Err(e.clone()),
            _ => Ok(None)
        }
    }

    pub fn to_result(&self, name: &str) -> crate::error::Result<T> {
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

    pub fn is_err(&self) -> bool {
        match self {
            LazyValue::Err(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JClassInfo {
    pub magic: LazyValue<u32>,
    pub minor_version: LazyValue<u16>,
    pub major_version: LazyValue<u16>,
    pub constant_pool: LazyValue<ConstantPool>,
    pub access_flags: LazyValue<u16>,
    pub class_index: LazyValue<u16>,
    pub superclass_index: LazyValue<u16>,
    pub interfaces: LazyValue<Vec<u16>>,
    pub fields: LazyValue<Vec<FieldInfo>>,
    pub methods: LazyValue<Vec<MethodInfo>>,
    pub attributes: LazyValue<Vec<AttributeInfo>>,
}

impl JClassInfo {
    pub fn magic(&self) -> Option<u32> {
        self.magic.get()
    }
    pub fn set_magic(&mut self, value: u32) {
        self.magic.some(value);
    }
    pub fn minor_version(&self) -> Option<u16> {
        self.minor_version.get()
    }
    pub fn set_minor_version(&mut self, value: u16) {
        self.minor_version.some(value);
    }
    pub fn major_version(&self) -> Option<u16> {
        self.major_version.get()
    }
    pub fn set_major_version(&mut self, value: u16) {
        self.major_version.some(value);
    }
    pub fn constant_pool(&self) -> Option<&ConstantPool> {
        self.constant_pool.get_ref()
    }
    pub fn set_constant_pool(&mut self, value: ConstantPool) {
        self.constant_pool.some(value);
    }
    pub fn access_flags(&self) -> Option<u16> {
        self.access_flags.get()
    }
    pub fn set_access_flags(&mut self, value: u16) {
        self.access_flags.some(value);
    }
    pub fn class_index(&self) -> Option<u16> {
        self.class_index.get()
    }
    pub fn set_class_index(&mut self, value: u16) {
        self.class_index.some(value);
    }
    pub fn superclass_index(&self) -> Option<u16> {
        self.superclass_index.get()
    }
    pub fn set_superclass_index(&mut self, value: u16) {
        self.superclass_index.some(value);
    }
    pub fn interfaces(&self) -> Option<&Vec<u16>> {
        self.interfaces.get_ref()
    }
    pub fn set_interfaces(&mut self, value: Vec<u16>) {
        self.interfaces.some(value);
    }
    pub fn fields(&self) -> Option<&Vec<FieldInfo>> {
        self.fields.get_ref()
    }
    pub fn set_fields(&mut self, value: Vec<FieldInfo>) {
        self.fields.some(value);
    }
    pub fn methods(&self) -> Option<&Vec<MethodInfo>> {
        self.methods.get_ref()
    }
    pub fn set_methods(&mut self, value: Vec<MethodInfo>) {
        self.methods.some(value);
    }
    pub fn attributes(&self) -> Option<&Vec<AttributeInfo>> {
        self.attributes.get_ref()
    }
    pub fn set_attributes(&mut self, value: Vec<AttributeInfo>) {
        self.attributes.some(value);
    }
}