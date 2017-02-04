/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

use std::{error, fmt};
use std::ascii::AsciiExt;

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnsupportedBase,
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
            UnsupportedBase => "Unsupported base",
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

/// List of supported bases.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Base {
    /// unary tends to be 11111
    Base1 = b'1' as isize,
    /// binary has 1 and 0
    Base2,
    /// highest char in octal
    Base8,
    /// highest char in decimal
    Base10,
    /// highest char in hex
    Base16,
    Base16Upper,
    /// rfc4648 no padding - highest char
    Base32hex,
    Base32hexUpper,
    /// rfc4648 with padding
    Base32hexpad,
    Base32hexpadUpper,
    /// rfc4648 no padding
    Base32,
    Base32Upper,
    /// rfc4648 with padding
    Base32pad,
    Base32padUpper,
    /// z-base-32 - used by Tahoe-LAFS - highest letter
    Base32z,
    /// highest letter
    Base58flickr,
    /// highest letter
    Base58btc,
    /// rfc4648 no padding
    Base64,
    /// rfc4648 with padding - MIME encoding
    Base64pad,
    /// rfc4648 no padding
    Base64url,
    /// rfc4648 with padding
    Base64urlpad,
}

impl Base {
    /// Get the base code.
    pub fn code(&self) -> u8 {
        use Base::*;

        match *self {
            Base1 => b'1',
            Base2 => b'0',
            Base8 => b'7',
            Base10 => b'9',
            Base16 => b'f',
            Base16Upper => b'F',
            Base32hex => b'v',
            Base32hexUpper => b'V',
            Base32hexpad => b't',
            Base32hexpadUpper => b'T',
            Base32 => b'b',
            Base32Upper => b'B',
            Base32pad => b'c',
            Base32padUpper => b'C',
            Base32z => b'h',
            Base58flickr => b'Z',
            Base58btc => b'z',
            Base64 => b'm',
            Base64pad => b'M',
            Base64url => b'u',
            Base64urlpad => b'U',
        }
    }

    /// Get the matching alphabet.
    pub fn alphabet(&self) -> Result<&[u8]> {
        use Base::*;

        Ok(match *self {
            Base1 => b"1",
            Base2 => b"01",
            Base8 => b"01234567",
            Base10 => b"0123456789",
            Base16 => b"0123456789abcdef",
            Base16Upper => b"0123456789ABCDEF",
            Base32hex => b"0123456789abcdefghijklmnopqrstuv",
            Base32hexUpper => b"0123456789ABCDEFGHIJKLMNOPQRSTUV",
            Base32hexpad => return Err(Error::UnsupportedBase),
            Base32hexpadUpper => return Err(Error::UnsupportedBase),
            Base32 => b"abcdefghijklmnopqrstuvwxyz234567",
            Base32Upper => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
            Base32pad => return Err(Error::UnsupportedBase),
            Base32padUpper => return Err(Error::UnsupportedBase),
            Base32z => b"ybndrfg8ejkmcpqxot1uwisza345h769",
            Base58flickr => b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ",
            Base58btc => b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            Base64 => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            Base64pad => return Err(Error::UnsupportedBase),
            Base64url => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            Base64urlpad => return Err(Error::UnsupportedBase),
        })
    }

    /// Convert a code to a base.
    pub fn from_code(code: u8) -> Result<Base> {
        use Base::*;

        match code {
            b'1' => Ok(Base1),
            b'0' => Ok(Base2),
            b'7' => Ok(Base8),
            b'9' => Ok(Base10),
            b'f' => Ok(Base16),
            b'F' => Ok(Base16Upper),
            b'v' => Ok(Base32hex),
            b'V' => Ok(Base32hexUpper),
            b't' => Ok(Base32hexpad),
            b'T' => Ok(Base32hexpadUpper),
            b'b' => Ok(Base32),
            b'B' => Ok(Base32Upper),
            b'c' => Ok(Base32pad),
            b'C' => Ok(Base32padUpper),
            b'h' => Ok(Base32z),
            b'Z' => Ok(Base58flickr),
            b'z' => Ok(Base58btc),
            b'm' => Ok(Base64),
            b'M' => Ok(Base64pad),
            b'u' => Ok(Base64url),
            b'U' => Ok(Base64urlpad),
            _ => Err(Error::UnkownBase),
        }
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
        if !self.is_ascii() {
            return Err(Error::InvalidBaseString);
        }
        let code = self.bytes().next().unwrap_or(0);
        let base = Base::from_code(code)?;
        let content = &self[1..];
        let alphabet = base.alphabet()?;
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
    fn encode(&self, base: Base) -> Result<String>;
}

impl Encodable for [u8] {
    #[inline]
    fn encode(&self, base: Base) -> Result<String> {
        let alphabet = base.alphabet()?;

        let mut encoded = base_x::encode(alphabet, self);
        encoded.insert(0, base.code() as char);
        Ok(encoded)
    }
}

impl<'a, E: AsRef<[u8]>> Encodable for E {
    #[inline]
    fn encode(&self, base: Base) -> Result<String> {
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
/// assert_eq!(encode(Base::Base58btc, b"hello").unwrap(),
///            "zCn8eVZg");
/// ```
pub fn encode<T: Encodable>(base: Base, data: T) -> Result<String> {
    data.encode(base)
}
