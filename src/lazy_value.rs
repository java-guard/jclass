use crate::error::MessageError;
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