//! The error module which exposes differnt error handling
//! enum members.
//!
use std::{fmt, error, io};

pub type RecolResult<T> = Result<T, RecolError>;

#[derive(Debug)]
pub enum RecolError {
    Io(io::Error),
    General(String),
}

impl error::Error for RecolError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => e.source(),
            Self::General(_) => None,
        }
    }
}

impl fmt::Display for RecolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::General(e) => f.write_str(&e)
        }
    }
}

impl From<io::Error> for RecolError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<io::ErrorKind> for RecolError {
    fn from(e: io::ErrorKind) -> Self {
        Self::Io(io::Error::from(e))
    }
}
