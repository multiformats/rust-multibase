use {Error, Result};

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
                    _ => Err(Error::UnkownBase),
                }
            }
        }
    }
}

build_base_enum! {
    // '1' => Base1: unimplemented!(),
    '0' => Base2: b"01",
    '7' => Base8: b"01234567",
    '9' => Base10: b"0123456789",
    'f' => Base16: b"0123456789abcdef",
    'F' => Base16Upper: b"0123456789ABCDEF",
    'v' => Base32hex: b"0123456789abcdefghijklmnopqrstuv",
    'V' => Base32hexUpper: b"0123456789ABCDEFGHIJKLMNOPQRSTUV",
    // 't' => Base32hexpad: unimplemented!(),
    // 'T' => Base32hexpadUpper: unimplemented!(),
    'b' => Base32: b"abcdefghijklmnopqrstuvwxyz234567",
    'B' => Base32Upper: b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    // 'c' => Base32pad: unimplemented!(),
    // 'C' => Base32padUpper: unimplemented!(),
    'h' => Base32z: b"ybndrfg8ejkmcpqxot1uwisza345h769",
    'Z' => Base58flickr: b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ",
    'z' => Base58btc: b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
    'm' => Base64: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    // 'M' => Base64pad: unimplemented!(),
    'u' => Base64url: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    // 'U' => Base64urlpad: unimplemented!(),
}
