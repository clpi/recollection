use std::{fmt, error, io};

pub type IchaResult<T> = Result<T, IchaError>;

#[derive(Debug)]
pub enum IchaError {
    IoError(io::Error),
    GeneralError(String),
}
impl error::Error for IchaError {}

impl fmt::Display for IchaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::GeneralError(e) => f.write_str(&e)
        }
    }
}
impl From<io::Error> for IchaError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
