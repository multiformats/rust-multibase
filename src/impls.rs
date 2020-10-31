use crate::encoding;
use crate::error::Result;

macro_rules! derive_base_encoding {
    ( $(#[$doc:meta] $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    $encoding.encode(input.as_ref())
                }

                fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
                    Ok($encoding.decode(input.as_ref().as_bytes())?)
                }
            }
        )*
    };
}

macro_rules! derive_base_x {
    ( $(#[$doc:meta] $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    base_x::encode($encoding, input.as_ref())
                }

                fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
                    Ok(base_x::decode($encoding, input.as_ref())?)
                }
            }
        )*
    };
}

pub(crate) trait BaseCodec {
    /// Encode with the given byte slice.
    fn encode<I: AsRef<[u8]>>(input: I) -> String;

    /// Decode with the given string.
    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>>;
}

/// Identity, 8-bit binary (encoder and decoder keeps data unmodified).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Identity;

impl BaseCodec for Identity {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        String::from_utf8(input.as_ref().to_vec()).expect("input must be valid UTF-8 bytes")
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(input.as_ref().as_bytes().to_vec())
    }
}

derive_base_encoding! {
    /// Base2 (alphabet: 01).
    Base2, encoding::BASE2;
    /// Base8 (alphabet: 01234567).
    Base8, encoding::BASE8;
    /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
    Base16Lower, encoding::BASE16_LOWER;
    /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
    Base16Upper, encoding::BASE16_UPPER;
    /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32Lower, encoding::BASE32_NOPAD_LOWER;
    /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32Upper, encoding::BASE32_NOPAD_UPPER;
    /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32PadLower, encoding::BASE32_PAD_LOWER;
    /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32PadUpper, encoding::BASE32_PAD_UPPER;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexLower, encoding::BASE32HEX_NOPAD_LOWER;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexUpper, encoding::BASE32HEX_NOPAD_UPPER;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexPadLower, encoding::BASE32HEX_PAD_LOWER;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexPadUpper, encoding::BASE32HEX_PAD_UPPER;
    /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    Base32Z, encoding::BASE32Z;
    /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64, encoding::BASE64_NOPAD;
    /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64Pad, encoding::BASE64_PAD;
    /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64Url, encoding::BASE64URL_NOPAD;
    /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64UrlPad, encoding::BASE64URL_PAD;
}

derive_base_x! {
    /// Base10 (alphabet: 0123456789).
    Base10, encoding::BASE10;
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    Base58Flickr, encoding::BASE58_FLICKR;
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    Base58Btc, encoding::BASE58_BITCOIN;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(Identity::encode(b"foo"), "foo");
        assert_eq!(Identity::decode("foo").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base2() {
        assert_eq!(Base2::encode(b"foo"), "011001100110111101101111");
        assert_eq!(
            Base2::decode("011001100110111101101111").unwrap(),
            b"foo".to_vec()
        );
    }

    #[test]
    fn test_base8() {
        assert_eq!(Base8::encode(b"foo"), "31467557");
        assert_eq!(Base8::decode("31467557").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base10() {
        assert_eq!(Base10::encode(b"foo"), "6713199");
        assert_eq!(Base10::decode("6713199").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base16() {
        assert_eq!(Base16Lower::encode(b"foo"), "666f6f");
        assert_eq!(Base16Lower::decode("666f6f").unwrap(), b"foo".to_vec());

        assert_eq!(Base16Upper::encode(b"foo"), "666F6F");
        assert_eq!(Base16Upper::decode("666F6F").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base32() {
        assert_eq!(Base32Lower::encode(b"foo"), "mzxw6");
        assert_eq!(Base32Lower::decode("mzxw6").unwrap(), b"foo".to_vec());

        assert_eq!(Base32Upper::encode(b"foo"), "MZXW6");
        assert_eq!(Base32Upper::decode("MZXW6").unwrap(), b"foo".to_vec());

        assert_eq!(Base32HexLower::encode(b"foo"), "cpnmu");
        assert_eq!(Base32HexLower::decode("cpnmu").unwrap(), b"foo".to_vec());

        assert_eq!(Base32HexUpper::encode(b"foo"), "CPNMU");
        assert_eq!(Base32HexUpper::decode("CPNMU").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base32_padding() {
        assert_eq!(Base32PadLower::encode(b"foo"), "mzxw6===");
        assert_eq!(Base32PadLower::decode("mzxw6===").unwrap(), b"foo".to_vec());

        assert_eq!(Base32PadUpper::encode(b"foo"), "MZXW6===");
        assert_eq!(Base32PadUpper::decode("MZXW6===").unwrap(), b"foo".to_vec());

        assert_eq!(Base32HexPadLower::encode(b"foo"), "cpnmu===");
        assert_eq!(
            Base32HexPadLower::decode("cpnmu===").unwrap(),
            b"foo".to_vec()
        );

        assert_eq!(Base32HexPadUpper::encode(b"foo"), "CPNMU===");
        assert_eq!(
            Base32HexPadUpper::decode("CPNMU===").unwrap(),
            b"foo".to_vec()
        );
    }

    #[test]
    fn test_base32z() {
        assert_eq!(Base32Z::encode(b"foo"), "c3zs6");
        assert_eq!(Base32Z::decode("c3zs6").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base58() {
        assert_eq!(Base58Flickr::encode(b"foo"), "ApAP");
        assert_eq!(Base58Flickr::decode("ApAP").unwrap(), b"foo".to_vec());

        assert_eq!(Base58Btc::encode(b"foo"), "bQbp");
        assert_eq!(Base58Btc::decode("bQbp").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base64() {
        assert_eq!(Base64::encode(b"foo"), "Zm9v");
        assert_eq!(Base64::decode("Zm9v").unwrap(), b"foo".to_vec());

        assert_eq!(Base64Url::encode(b"foo"), "Zm9v");
        assert_eq!(Base64Url::decode("Zm9v").unwrap(), b"foo".to_vec());
    }

    #[test]
    fn test_base64_padding() {
        assert_eq!(Base64Pad::encode(b"foopadding"), "Zm9vcGFkZGluZw==");
        assert_eq!(
            Base64Pad::decode("Zm9vcGFkZGluZw==").unwrap(),
            b"foopadding".to_vec()
        );

        assert_eq!(Base64UrlPad::encode(b"foopadding"), "Zm9vcGFkZGluZw==");
        assert_eq!(
            Base64UrlPad::decode("Zm9vcGFkZGluZw==").unwrap(),
            b"foopadding".to_vec()
        );
    }
}
