/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

use std::panic;

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnsupportedBase,
    UnkownBase,
    Utf8Error,
    InvalidBaseString,
}

/// Encoding result type
pub type EncodeResult = Result<String, Error>;

/// Decoding result type
pub type DecodeResult = Result<(Base, Vec<u8>), Error>;

/// List of supported bases.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Base {
    /// unary tends to be 11111
    Base1,
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
    pub fn code(&self) -> &str {
        use Base::*;

        match *self {
            Base1 => "1",
            Base2 => "0",
            Base8 => "7",
            Base10 => "9",
            Base16 => "f",
            Base16Upper => "F",
            Base32hex => "v",
            Base32hexUpper => "V",
            Base32hexpad => "t",
            Base32hexpadUpper => "T",
            Base32 => "b",
            Base32Upper => "B",
            Base32pad => "c",
            Base32padUpper => "C",
            Base32z => "h",
            Base58flickr => "Z",
            Base58btc => "z",
            Base64 => "m",
            Base64pad => "M",
            Base64url => "u",
            Base64urlpad => "U",
        }
    }

    /// Get the matching alphabet.
    pub fn alphabet(&self) -> Result<String, Error> {
        use Base::*;

        match *self {
            Base1 => Ok("1".to_string()),
            Base2 => Ok("01".to_string()),
            Base8 => Ok("01234567".to_string()),
            Base10 => Ok("0123456789".to_string()),
            Base16 => Ok("0123456789abcdef".to_string()),
            Base16Upper => Ok("0123456789ABCDEF".to_string().to_string()),
            Base32hex => Ok("0123456789abcdefghijklmnopqrstuv".to_string()),
            Base32hexUpper => Ok("0123456789ABCDEFGHIJKLMNOPQRSTUV".to_string()),
            Base32hexpad => Err(Error::UnsupportedBase),
            Base32hexpadUpper => Err(Error::UnsupportedBase),
            Base32 => Ok("abcdefghijklmnopqrstuvwxyz234567".to_string()),
            Base32Upper => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".to_string()),
            Base32pad => Err(Error::UnsupportedBase),
            Base32padUpper => Err(Error::UnsupportedBase),
            Base32z => Ok("ybndrfg8ejkmcpqxot1uwisza345h769".to_string()),
            Base58flickr => {
                Ok("123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ".to_string())
            }
            Base58btc => {
                Ok("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())
            }
            Base64 => {
                Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string())
            }
            Base64pad => Err(Error::UnsupportedBase),
            Base64url => {
                Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string())
            }
            Base64urlpad => Err(Error::UnsupportedBase),
        }
    }

    /// Convert a code to a base.
    pub fn from_code(code: &str) -> Result<Base, Error> {
        use Base::*;

        match code {
            "1" => Ok(Base1),
            "0" => Ok(Base2),
            "7" => Ok(Base8),
            "9" => Ok(Base10),
            "f" => Ok(Base16),
            "F" => Ok(Base16Upper),
            "v" => Ok(Base32hex),
            "V" => Ok(Base32hexUpper),
            "t" => Ok(Base32hexpad),
            "T" => Ok(Base32hexpadUpper),
            "b" => Ok(Base32),
            "B" => Ok(Base32Upper),
            "c" => Ok(Base32pad),
            "C" => Ok(Base32padUpper),
            "h" => Ok(Base32z),
            "Z" => Ok(Base58flickr),
            "z" => Ok(Base58btc),
            "m" => Ok(Base64),
            "M" => Ok(Base64pad),
            "u" => Ok(Base64url),
            "U" => Ok(Base64urlpad),
            _ => Err(Error::UnkownBase),
        }
    }
}

pub trait Decodable {
    fn decode(&self) -> DecodeResult;
}

/// Decode the string.
pub fn decode<T: Decodable>(data: T) -> DecodeResult {
    data.decode()
}

/// base_x panics on invalid input
fn safe_decode(alphabet: &str, content: &str) -> std::thread::Result<Vec<i16>> {
    panic::catch_unwind(|| base_x::decode(alphabet, content))
}

impl<'a> Decodable for &'a str {
    fn decode(&self) -> DecodeResult {
        let (base_char, content) = self.split_at(1);

        Base::from_code(base_char).and_then(|base| {
            base.alphabet()
                .and_then(|alphabet| {
                    safe_decode(&alphabet, content).map_err(|_| Error::InvalidBaseString)
                })
                .map(|decoded| {
                    let res = decoded.iter()
                        .map(|u| *u as u8)
                        .collect();
                    (base, res)
                })
        })
    }
}

impl Decodable for String {
    fn decode(&self) -> DecodeResult {
        (&self[..]).decode()
    }
}

impl<'a> Decodable for &'a [u8] {
    fn decode(&self) -> DecodeResult {
        match std::str::from_utf8(self) {
            Ok(string) => string.decode(),
            Err(_) => Err(Error::Utf8Error),
        }
    }
}


pub trait Encodable {
    /// Encode with the given base
    fn encode(&self, base: Base) -> EncodeResult;
}

impl<'a> Encodable for &'a str {
    fn encode(&self, base: Base) -> EncodeResult {
        base.alphabet().map(|alphabet| {
            let chars: Vec<i16> = self.encode_utf16()
                .map((|u| u as i16))
                .collect();

            println!("in {:?}", chars);
            base.code().to_string() + &base_x::encode(&alphabet, chars)
        })
    }
}

impl Encodable for String {
    fn encode(&self, base: Base) -> EncodeResult {
        (&self[..]).encode(base)
    }
}

impl<'a> Encodable for &'a [u8] {
    fn encode(&self, base: Base) -> EncodeResult {
        base.alphabet().map(|alphabet| {
            let chars = self.to_vec()
                .iter()
                .map(|u| *u as i16)
                .collect();
            base.code().to_string() + &base_x::encode(&alphabet, chars)
        })
    }
}

impl Encodable for Vec<u8> {
    fn encode(&self, base: Base) -> EncodeResult {
        self.as_slice().encode(base)
    }
}

/// Encode with the given string
pub fn encode<T: Encodable>(base: Base, data: T) -> EncodeResult {
    data.encode(base)
}
