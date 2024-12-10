use std::fmt::{Debug, Display, Formatter};
use std::io::Read;

pub type Reader = Box<dyn Read>;

#[derive(Debug, Clone)]
pub struct MessageError {
    msg: String
}

impl MessageError {
    pub fn new(msg: &str) -> MessageError {
        MessageError {
            msg: String::from(msg)
        }
    }
}

impl<T> Into<Result<T>> for MessageError {
    fn into(self) -> Result<T> {
        Err(self)
    }
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for MessageError {

}

pub type Result<T> = core::result::Result<T, MessageError>;

pub trait ToResult<T> {
    fn with_message(self, msg: &str) -> Result<T>;
}

impl<T, E: Display> ToResult<T> for core::result::Result<T, E> {
    fn with_message(self, msg: &str) -> Result<T> {
        self.map_err(|e| MessageError::new(&format!("{msg}: {e}")))
    }
}
