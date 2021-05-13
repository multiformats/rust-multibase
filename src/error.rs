use core::fmt;

/// Type alias to use this library's [`Error`] type in a `Result`.
pub type Result<T> = core::result::Result<T, Error>;

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// Unknown base code.
    UnknownBase(char),
    /// Invalid string.
    InvalidBaseString,
    /// Decode Err
    DecodeError(data_encoding::DecodeError),
    /// Encoding/Decode Failed
    WriteFail(data_encoding::DecodePartial),
    /// Mismatched sizes
    MismatchedSizes(usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownBase(code) => write!(f, "Unknown base code: {}", code),
            Error::InvalidBaseString => write!(f, "Invalid base string"),
            Error::DecodeError(err) => write!(f, "Error decoding something: {:?}", err),
            Error::WriteFail(partial) => write!(f, "Partial Decoding: {:?}", partial),
            Error::MismatchedSizes(input, output) => write!(
                f,
                "Input and output slices are different sizes {}:{}",
                input, output
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "alloc")]
impl From<base_x::DecodeError> for Error {
    fn from(_: base_x::DecodeError) -> Self {
        Self::InvalidBaseString
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(_: data_encoding::DecodeError) -> Self {
        Self::InvalidBaseString
    }
}

impl From<data_encoding::DecodePartial> for Error {
    fn from(err: data_encoding::DecodePartial) -> Self {
        Self::WriteFail(err)
    }
}
