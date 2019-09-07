use base_x;
use std::{error, fmt};

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnknownBase,
    InvalidBaseString,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            UnknownBase => "Unknown base",
            InvalidBaseString => "Invalid base string",
        }
    }
}

impl From<base_x::DecodeError> for Error {
    fn from(_: base_x::DecodeError) -> Self {
        Self::InvalidBaseString
    }
}

impl From<base64::DecodeError> for Error {
    fn from(_: base64::DecodeError) -> Self {
        Self::InvalidBaseString
    }
}
