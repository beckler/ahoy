use std::{error::Error, fmt::Display};

pub mod device;
pub mod list;

#[derive(Debug, Clone)]
pub enum CommandError {
    JSONError(String),
    DeviceError(String),
    RetievalError(String),
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
