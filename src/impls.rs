use crate::encoding;
use crate::error::Result;

pub use big::*;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

macro_rules! derive_base_encoding {
    ( $(#[$doc:meta] $code:literal => $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec<String, Vec<u8>> for $type {
                const CODE: char = $code;

                type Error = crate::Error;

                fn encode(input: impl AsRef<[u8]>) -> Result<String> {
                    Ok($encoding.encode(input.as_ref()))
                }

                fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
                    Ok($encoding.decode(input.as_ref().as_bytes())?)
                }
            }
            
        )*
    };
}

macro_rules! derive_base_x {
    ( $(#[$doc:meta] $code:literal => $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec<String, Vec<u8>> for $type {
                const CODE: char = $code;

                type Error = crate::Error;

                fn encode(input: impl AsRef<[u8]>) -> Result<String> {
                    Ok(base_x::encode($encoding, input.as_ref()))
                }

                fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
                    Ok(base_x::decode($encoding, input.as_ref())?)
                }
            }
        )*
    };
}

pub(crate) trait BaseCodec<En, De> {
    const CODE: char;
    
    /// dont matter xd
    type Error;
    
    /// Encode with the given byte slice.
    fn encode(input: impl AsRef<[u8]>) -> core::result::Result<En, Self::Error>;
    /// Decode with the given string (as slice).
    fn decode(input: impl AsRef<str>) -> core::result::Result<De, Self::Error>;

    fn code(&self) -> char {
        Self::CODE
    }
}

/// Identity, 8-bit binary (encoder and decoder keeps data unmodified).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Identity;

impl BaseCodec<String, Vec<u8>> for Identity {
    const CODE: char = '\x00';

    type Error = crate::Error;

    fn encode(input: impl AsRef<[u8]>) -> Result<String> {
        String::from_utf8(input.as_ref().to_vec()).map_err(|e| crate::Error::InvalidBaseString)
    }

    fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
        Ok(input.as_ref().as_bytes().to_vec())
    }
}

derive_base_encoding! {
    /// Base2 (alphabet: 01).
    '0' => Base2, encoding::BASE2;
    /// Base8 (alphabet: 01234567).
    '7' => Base8, encoding::BASE8;
    /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
    'f' => Base16Lower, encoding::BASE16_LOWER;
    /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
    'F' => Base16Upper, encoding::BASE16_UPPER;
    /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    'b' => Base32Lower, encoding::BASE32_NOPAD_LOWER;
    /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    'B' => Base32Upper, encoding::BASE32_NOPAD_UPPER;
    /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    'c' => Base32PadLower, encoding::BASE32_PAD_LOWER;
    /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    'C' => Base32PadUpper, encoding::BASE32_PAD_UPPER;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    'v' => Base32HexLower, encoding::BASE32HEX_NOPAD_LOWER;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    'V' => Base32HexUpper, encoding::BASE32HEX_NOPAD_UPPER;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    't' => Base32HexPadLower, encoding::BASE32HEX_PAD_LOWER;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    'T' => Base32HexPadUpper, encoding::BASE32HEX_PAD_UPPER;
    /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    'h' => Base32Z, encoding::BASE32Z;
    /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    'm' => Base64, encoding::BASE64_NOPAD;
    /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    'M' => Base64Pad, encoding::BASE64_PAD;
    /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    'u' => Base64Url, encoding::BASE64URL_NOPAD;
    /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    'U' => Base64UrlPad, encoding::BASE64URL_PAD;
}

derive_base_x! {
    /// Base10 (alphabet: 0123456789).
    '9' => Base10, encoding::BASE10;
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    'Z' => Base58Flickr, encoding::BASE58_FLICKR;
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    'z' => Base58Btc, encoding::BASE58_BITCOIN;
}


mod big {
    use base_x;

    use crate::encoding;

    use super::{BaseCodec, Result};
    /// Base36, [0-9a-z] no padding (alphabet: abcdefghijklmnopqrstuvwxyz0123456789).
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub(crate) struct Base36Lower;

    impl BaseCodec<String, Vec<u8>> for Base36Lower {
        const CODE: char = 'k';

        type Error = crate::Error;

        fn encode(input: impl AsRef<[u8]>) -> Result<String> {
            Ok(base_x::encode(encoding::BASE36_LOWER, input.as_ref()))
        }

        fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
            // The input is case insensitive, hence lowercase it
            let lowercased = input.as_ref().to_ascii_lowercase();
            Ok(base_x::decode(encoding::BASE36_LOWER, &lowercased)?)
        }
    }

    /// Base36, [0-9A-Z] no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789).
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub(crate) struct Base36Upper;

    impl BaseCodec<String, Vec<u8>> for Base36Upper {
        const CODE: char = 'K';

        type Error = crate::Error;

        fn encode(input: impl AsRef<[u8]>) -> Result<String> {
            Ok(base_x::encode(encoding::BASE36_UPPER, input.as_ref()))
        }

        fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
            // The input is case insensitive, hence uppercase it
            let uppercased = input.as_ref().to_ascii_uppercase();
            Ok(base_x::decode(encoding::BASE36_UPPER, &uppercased)?)
        }
    }
}

// mod smol {
//         use smol_base_x::Base;

//         use super::BaseCodec;
//         use super::Result;
//         /// Base36, [0-9a-z] no padding (alphabet: abcdefghijklmnopqrstuvwxyz0123456789).
//         #[derive(PartialEq, Eq, Clone, Copy, Debug)]
//         pub(crate) struct Base36Lower<const S: usize>;

//         impl<const S: usize> Base<36> for Base36Lower<S> {
//             const ALPHABET: [u8; 36] = *b"abcdefghijklmnopqrstuvwxyz0123456789";
//         }
    
//         impl<const S: usize> BaseCodec for Base36Lower<S> {
//             fn encode<I: AsRef<[u8]>>(input: I) -> String {
//                 let mut buf = [0u8; S];
//                 let written = Base36Lower::encode_mut(input, &mut buf).unwrap();
//                 String::from_utf8(buf[..written].to_vec()).unwrap()
//             }
    
//             fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
//                 // The input is case insensitive, hence lowercase it
//                 let lowercased = input.as_ref().to_ascii_lowercase();
//                 let mut buf = [0u8; S];
//                 let written = Base36Lower::decode_mut(input.as_ref(), &mut buf).unwrap();

//                 Ok(buf[..written].to_owned())
//             }
//         }
// }