use crate::error::{Error, Result};
use crate::impls::*;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

macro_rules! build_base_enum {
    ( $(#[$attr:meta] $base:ident,)* ) => {
        /// List of types currently supported in the multibase spec.
        ///
        /// Not all base types are supported by this library.
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum Base {
            $( #[$attr] $base, )*
        }
        impl Base {
            /// Convert a number to the matching base algorithm, or `Error` if no algorithm is matching.
            pub fn from_code(code: char) -> Result<Self> {
        	    match code {
                    $( $base::CODE => Ok(Self::$base), )*
            	    _ => Err(Error::UnknownBase(code)),
        	    }
            }

            /// Get the code corresponding to the base algorithm.
            pub fn code(&self) -> char {
                match self {
                    $( Self::$base => $base::CODE, )*
                }
            }

            /// Encode the given byte slice to base string.
            pub fn encode<I: AsRef<[u8]>>(&self, input: I) -> String {
                match self {
                    $( Self::$base => $base::encode(input).unwrap(), )* // encode wont panic for String
                }
            }

            /// Decode the base string.
            pub fn decode<I: AsRef<str>>(&self, input: I) -> Result<Vec<u8>> {
                match self {
                    $( Self::$base => $base::decode(input), )*
                }
            }
        }

        
    }
}

build_base_enum! {
    /// 8-bit binary (encoder and decoder keeps data unmodified).
    Identity,
    /// Base2 (alphabet: 01).
    Base2,
    /// Base8 (alphabet: 01234567).
    Base8,
    /// Base10 (alphabet: 0123456789).
    Base10,
    /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
    Base16Lower,
    /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
    Base16Upper,
     /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32Lower,
    /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32Upper,
    /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32PadLower,
    /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32PadUpper,
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexLower,
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexUpper,
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexPadLower,
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexPadUpper,
    /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    Base32Z,
    /// Base36, [0-9a-z] no padding (alphabet: 0123456789abcdefghijklmnopqrstuvwxyz).
    Base36Lower,
    /// Base36, [0-9A-Z] no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).
    Base36Upper,
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    Base58Flickr,
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    Base58Btc,
    /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64,
    /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64Pad,
    /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64Url,
    /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64UrlPad,
}
