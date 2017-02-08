/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

mod base;

pub use base::Base;
pub use Base::*;

use std::{error, fmt};

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnkownBase,
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
            UnkownBase => "Unkown base",
            InvalidBaseString => "Invalid base string",
        }
    }
}

impl From<base_x::DecodeError> for Error {
    fn from(_: base_x::DecodeError) -> Error {
        Error::InvalidBaseString
    }
}

pub trait Decodable {
    fn decode(&self) -> Result<(Base, Vec<u8>)>;
}

/// Decode the string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, decode};
///
/// assert_eq!(decode("zCn8eVZg").unwrap(),
///            (Base::Base58btc, b"hello".to_vec()));
/// ```
    #[inline]
pub fn decode<T: Decodable>(data: T) -> Result<(Base, Vec<u8>)> {
    data.decode()
}

impl Decodable for str {
    fn decode(&self) -> Result<(Base, Vec<u8>)> {
        let code = self.chars().next().ok_or(Error::InvalidBaseString)?;
        let base = Base::from_code(code)?;
        let content = &self[code.len_utf8()..];
        let alphabet = base.alphabet();
        let decoded = base_x::decode(alphabet, content)?;
        Ok((base, decoded))
     }
}

impl<'a, D: AsRef<str>> Decodable for D {
    #[inline]
    fn decode(&self) -> Result<(Base, Vec<u8>)> {
        self.as_ref().decode()
    }
}

pub trait Encodable {
    /// Encode with the given base
    fn encode(&self, base: Base) -> String;
}

impl Encodable for [u8] {
    #[inline]
    fn encode(&self, base: Base) -> String {
        let alphabet = base.alphabet();

        let mut encoded = base_x::encode(alphabet, self);
        encoded.insert(0, base.code());
        encoded
    }
}

impl<'a, E: AsRef<[u8]>> Encodable for E {
    #[inline]
    fn encode(&self, base: Base) -> String {
        self.as_ref().encode(base)
    }
}

/// Encode with the given string
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode};
///
/// assert_eq!(encode(Base::Base58btc, b"hello"),
///            "zCn8eVZg");
/// ```
pub fn encode<T: Encodable>(base: Base, data: T) -> String {
    data.encode(base)
}
