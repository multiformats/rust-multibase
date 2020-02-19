use crate::{Error, Result};
use data_encoding::{BASE32_NOPAD, BASE32, BASE64, BASE64_NOPAD, BASE64URL, BASE64URL_NOPAD};

trait BaseImpl {
    /// Encode a byte slice.
    fn encode(input: &[u8]) -> String;

    /// Decode a string.
    fn decode(input: &str) -> Result<Vec<u8>>;
}

macro_rules! base_x {
    ($name:ident, $alphabet:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name;

        impl $name {
            /// Get the matching alphabet.
            fn alphabet() -> &'static [u8] {
                $alphabet
            }
        }

        impl BaseImpl for $name {
            fn encode(input: &[u8]) -> String {
                let alphabet = Self::alphabet();
                base_x::encode(alphabet, input)
            }

            fn decode(input: &str) -> Result<Vec<u8>> {
                let alphabet = Self::alphabet();
                let decoded = base_x::decode(alphabet, input)?;
                Ok(decoded)
            }
        }
    };
}

macro_rules! base_enum {
    ( $($code:expr => $base:ident,)* ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Base {
            $( $base, )*
        }

        impl Base {
            pub fn from_code(code: char) -> Result<Self> {
        	match code {
                    $( $code => Ok(Self::$base), )*
            	    _ => Err(Error::UnknownBase),
        	}
            }

            pub fn code(&self) -> char {
                match self {
                    $( Self::$base => $code, )*
                }
            }

            pub fn encode(&self, input: &[u8]) -> String {
                match self {
                    $( Self::$base => $base::encode(input), )*
                }
            }

            pub fn decode(&self, input: &str) -> Result<Vec<u8>> {
                match self {
                    $( Self::$base => $base::decode(input), )*
                }
            }
        }
    }
}

// binary has 1 and 0
base_x!(Base2, b"01");
// highest char in octal
base_x!(Base8, b"01234567");
// highest char in decimal
base_x!(Base10, b"0123456789");
// highest char in hex
base_x!(Base16Upper, b"0123456789ABCDEF");
base_x!(Base16Lower, b"0123456789abcdef");
// highest letter
base_x!(
    Base58flickr,
    b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
);
// highest letter
base_x!(
    Base58btc,
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
);

/// rfc4648 no padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base32UpperNoPad;

impl BaseImpl for Base32UpperNoPad {
    fn encode(input: &[u8]) -> String {
        BASE32_NOPAD.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE32_NOPAD.decode(input.as_bytes())?)
    }
}

/// rfc4648 with padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base32UpperPad;

impl BaseImpl for Base32UpperPad {
    fn encode(input: &[u8]) -> String {
        BASE32.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE32.decode(input.as_bytes())?)
    }
}

/// rfc4648 no padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base64UpperNoPad;

impl BaseImpl for Base64UpperNoPad {
    fn encode(input: &[u8]) -> String {
        BASE64_NOPAD.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE64_NOPAD.decode(input.as_bytes())?)
    }
}

/// rfc4648 with padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base64UpperPad;

impl BaseImpl for Base64UpperPad {
    fn encode(input: &[u8]) -> String {
        BASE64.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE64.decode(input.as_bytes())?)
    }
}

/// rfc4648 no padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base64UrlUpperNoPad;

impl BaseImpl for Base64UrlUpperNoPad {
    fn encode(input: &[u8]) -> String {
        BASE64URL_NOPAD.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE64URL_NOPAD.decode(input.as_bytes())?)
    }
}

/// rfc4648 with padding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base64UrlUpperPad;

impl BaseImpl for Base64UrlUpperPad {
    fn encode(input: &[u8]) -> String {
        BASE64URL.encode(input)
    }

    fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(BASE64URL.decode(input.as_bytes())?)
    }
}

base_enum! {
    '0' => Base2,
    '7' => Base8,
    '9' => Base10,
    'F' => Base16Upper,
    'f' => Base16Lower,
    'B' => Base32UpperNoPad,
    'C' => Base32UpperPad,
    'Z' => Base58flickr,
    'z' => Base58btc,
    'm' => Base64UpperNoPad,
    'M' => Base64UpperPad,
    'u' => Base64UrlUpperNoPad,
    'U' => Base64UrlUpperPad,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base2() {
        assert_eq!(Base2::encode(b"f"), "1100110");
        assert_eq!(&Base2::decode("1100110").unwrap(), b"f");
    }

    #[test]
    fn test_base16() {
        assert_eq!(Base16Lower::encode(b"f"), "66");
        assert_eq!(&Base16Lower::decode("66").unwrap(), b"f");
    }

    #[test]
    fn test_base32() {
        assert_eq!(Base32UpperNoPad::encode(b"f"), "MY");
        assert_eq!(&Base32UpperNoPad::decode("MY").unwrap(), b"f");
    }

    #[test]
    fn test_base58() {
        assert_eq!(Base58btc::encode(b"f"), "2m");
        assert_eq!(&Base58btc::decode("2m").unwrap(), b"f");
    }

    #[test]
    fn test_base64() {
        assert_eq!(Base64UpperNoPad::encode(b"f"), "Zg");
        assert_eq!(&Base64UpperNoPad::decode("Zg").unwrap(), b"f");
    }

    #[test]
    fn test_encode_padding() {
        assert_eq!(Base32UpperNoPad::encode(b"foo"), "MZXW6");
        assert_eq!(Base32UpperPad::encode(b"foo"), "MZXW6===");

        assert_eq!(Base32UpperNoPad::encode(b"foob"), "MZXW6YQ");
        assert_eq!(Base32UpperPad::encode(b"foob"), "MZXW6YQ=");

        assert_eq!(Base32UpperNoPad::encode(b"fooba"), "MZXW6YTB");
        assert_eq!(Base32UpperPad::encode(b"fooba"), "MZXW6YTB");

        assert_eq!(Base32UpperNoPad::encode(b"foobar"), "MZXW6YTBOI");
        assert_eq!(Base32UpperPad::encode(b"foobar"), "MZXW6YTBOI======");
    }

    #[test]
    fn test_decode_padding() {
        assert_eq!(&Base32UpperNoPad::decode("MZXW6").unwrap(), b"foo");
        assert_eq!(&Base32UpperPad::decode("MZXW6===").unwrap(), b"foo");

        assert_eq!(&Base32UpperNoPad::decode("MZXW6YQ").unwrap(), b"foob");
        assert_eq!(&Base32UpperPad::decode("MZXW6YQ=").unwrap(), b"foob");

        assert_eq!(&Base32UpperNoPad::decode("MZXW6YTB").unwrap(), b"fooba");
        assert_eq!(&Base32UpperPad::decode("MZXW6YTB").unwrap(), b"fooba");

        assert_eq!(&Base32UpperNoPad::decode("MZXW6YTBOI").unwrap(), b"foobar");
        assert_eq!(&Base32UpperPad::decode("MZXW6YTBOI=====").unwrap(), b"foobar");
    }
}
