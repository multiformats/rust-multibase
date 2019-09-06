use crate::{Error, Result};

macro_rules! build_base_enum {
    {$( $val:expr => $var:ident: $alph:expr, )*} => {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum Base {
            $( $var, )*
        }

        use Base::*;

        impl Base {
            /// Get the base code.
            pub fn code(&self) -> char {
                match *self {
                    $( $var => $val, )*
                }
            }

            /// Get the matching alphabet.
            pub fn alphabet(&self) -> &[u8] {
                match *self {
                    $( $var => $alph, )*
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: char) -> Result<Base> {
                match code {
                    $( $val => Ok($var), )*
                    _ => Err(Error::UnknownBase),
                }
            }
        }
    }
}

build_base_enum! {
    // unary tends to be 11111
    // '1' => Base1: unimplemented!(),

    // binary has 1 and 0
    '0' => Base2: b"01",

    // highest char in octal
    '7' => Base8: b"01234567",

    // highest char in decimal
    '9' => Base10: b"0123456789",

    // highest char in hex
    'f' => Base16: b"0123456789abcdef",
    'F' => Base16Upper: b"0123456789ABCDEF",

    // rfc4648 no padding - highest char
    'v' => Base32hex: b"0123456789abcdefghijklmnopqrstuv",
    'V' => Base32hexUpper: b"0123456789ABCDEFGHIJKLMNOPQRSTUV",

    // rfc4648 with padding
    // 't' => Base32hexpad: unimplemented!(),
    // 'T' => Base32hexpadUpper: unimplemented!(),

    // rfc4648 no padding
    'b' => Base32: b"abcdefghijklmnopqrstuvwxyz234567",
    'B' => Base32Upper: b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",

    // rfc4648 with padding
    // 'c' => Base32pad: unimplemented!(),
    // 'C' => Base32padUpper: unimplemented!(),

    // z-base-32 - used by Tahoe-LAFS - highest letter
    'h' => Base32z: b"ybndrfg8ejkmcpqxot1uwisza345h769",

    // highest letter
    'Z' => Base58flickr: b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ",

    // highest letter
    'z' => Base58btc: b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",

    // rfc4648 no padding
    'm' => Base64: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",

    // rfc4648 with padding - MIME encoding
    'M' => Base64pad: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",

    // rfc4648 no padding
    'u' => Base64url: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",

    // rfc4648 with padding
    'U' => Base64urlpad: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
}
