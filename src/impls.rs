use crate::encoding;
use crate::error::Result;

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

/// Base2 (alphabet: 01).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base2;

impl BaseCodec for Base2 {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE2.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE2.decode(input.as_ref().as_bytes())?)
    }
}

/// Base8 (alphabet: 01234567).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base8;

impl BaseCodec for Base8 {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE8.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE8.decode(input.as_ref().as_bytes())?)
    }
}

/// Base10 (alphabet: 0123456789).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base10;

impl BaseCodec for Base10 {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE10, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(base_x::decode(encoding::BASE10, input.as_ref())?)
    }
}

/// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base16Lower;

impl BaseCodec for Base16Lower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE16_LOWER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE16_LOWER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base16Upper;

impl BaseCodec for Base16Upper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE16_UPPER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE16_UPPER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32Lower;

impl BaseCodec for Base32Lower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32_NOPAD_LOWER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32_NOPAD_LOWER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32Upper;

impl BaseCodec for Base32Upper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32_NOPAD_UPPER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32_NOPAD_UPPER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32PadLower;

impl BaseCodec for Base32PadLower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32_PAD_LOWER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32_PAD_LOWER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32PadUpper;

impl BaseCodec for Base32PadUpper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32_PAD_UPPER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32_PAD_UPPER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32HexLower;

impl BaseCodec for Base32HexLower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32HEX_NOPAD_LOWER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32HEX_NOPAD_LOWER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32HexUpper;

impl BaseCodec for Base32HexUpper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32HEX_NOPAD_UPPER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32HEX_NOPAD_UPPER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32HexPadLower;

impl BaseCodec for Base32HexPadLower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32HEX_PAD_LOWER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32HEX_PAD_LOWER.decode(input.as_ref().as_bytes())?)
    }
}

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32HexPadUpper;

impl BaseCodec for Base32HexPadUpper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32HEX_PAD_UPPER.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32HEX_PAD_UPPER.decode(input.as_ref().as_bytes())?)
    }
}

/// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base32Z;

impl BaseCodec for Base32Z {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE32Z.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE32Z.decode(input.as_ref().as_bytes())?)
    }
}

/// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base58Flickr;

impl BaseCodec for Base58Flickr {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE58_FLICKR, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(base_x::decode(encoding::BASE58_FLICKR, input.as_ref())?)
    }
}

/// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base58Btc;

impl BaseCodec for Base58Btc {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE58_BITCOIN, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(base_x::decode(encoding::BASE58_BITCOIN, input.as_ref())?)
    }
}

/// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base64;

impl BaseCodec for Base64 {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE64_NOPAD.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE64_NOPAD.decode(input.as_ref().as_bytes())?)
    }
}

/// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base64Pad;

impl BaseCodec for Base64Pad {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE64_PAD.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE64_PAD.decode(input.as_ref().as_bytes())?)
    }
}

/// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base64Url;

impl BaseCodec for Base64Url {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE64URL_NOPAD.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE64URL_NOPAD.decode(input.as_ref().as_bytes())?)
    }
}

/// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base64UrlPad;

impl BaseCodec for Base64UrlPad {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        encoding::BASE64URL_PAD.encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(encoding::BASE64URL_PAD.decode(input.as_ref().as_bytes())?)
    }
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
