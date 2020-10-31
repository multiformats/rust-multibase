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
