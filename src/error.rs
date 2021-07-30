//! The error module which exposes differnt error handling
//! enum members.
//!
use std::{fmt, error, io};

pub type RecolResult<T> = Result<T, RecolError>;

#[derive(Debug)]
pub enum RecolError {
    IoError(io::Error),
    GeneralError(String),
}
impl error::Error for RecolError {}

impl fmt::Display for RecolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::GeneralError(e) => f.write_str(&e)
        }
    }
}
impl From<io::Error> for RecolError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
