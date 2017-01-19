/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnsupportedBase,
    UnkownBase,
}

/// Encoding result type
pub type EncodeResult = Result<String, Error>;

/// Decoding result type
pub type DecodeResult = Result<(Base, String), Error>;

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

fn vec_to_string(vector: Vec<i16>) -> String {
    vector.into_iter()
        .map(|u| String::from_utf16(&[u as u16]).unwrap())
        .collect::<String>()
}


/// Encode a given string with the specified base.
pub fn encode(base: Base, data: &str) -> EncodeResult {
    base.alphabet().map(|alphabet| {
        let chars: Vec<i16> = data.encode_utf16()
            .map((|u| u as i16))
            .collect();
        base.code().to_string() + &base_x::encode(&alphabet, chars)
    })
}

/// Decode the string.
pub fn decode(data: &str) -> DecodeResult {
    let (base_char, content) = data.split_at(1);

    Base::from_code(base_char).and_then(|base| {
        base.alphabet().map(|alphabet| {
            let res = base_x::decode(&alphabet, content);
            (base, vec_to_string(res))
        })
    })
}
