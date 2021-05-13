use crate::encoding;
use crate::error::{Error, Result};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

macro_rules! derive_base_encoding {
    ( $(#[$doc:meta] $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                #[cfg(feature = "alloc")]
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    $encoding.encode(input.as_ref())
                }

                #[cfg(feature = "alloc")]
                fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
                    Ok($encoding.decode(input.as_ref().as_bytes())?)
                }

                fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]){
                    $encoding.encode_mut(input.as_ref(), output)
                }

                fn encode_len(len: usize) -> usize {
                    $encoding.encode_len(len)
                }

                fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize> {
                    let input_len = $encoding.decode_len(input.as_ref().len())?;
                    if input_len != output.len() {
                        return Err(Error::MismatchedSizes(input_len, output.len()))
                    }
                    $encoding.decode_mut(input.as_ref().as_bytes(), output).map_err(Error::WriteFail)
                }

                fn decode_len(len: usize) -> Result<usize> {
                    $encoding.decode_len(len).map_err(Error::DecodeError)
                }
            }
        )*
    };
}

#[cfg(feature = "alloc")]
macro_rules! derive_base_x {
    ( $(#[$doc:meta] $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    base_x::encode($encoding, input.as_ref())
                }

                fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
                    Ok(base_x::decode($encoding, input.as_ref())?)
                }

                fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]){
                    let out = base_x::encode($encoding, input.as_ref());
                    output[..out.len()].copy_from_slice(out.as_bytes());
                }

                fn encode_len(len: usize) -> usize {
                    encoding::calc_encoded_size($encoding.chars().count(), len)
                }

                fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize> {
                    let out = base_x::decode($encoding, input.as_ref())?;
                    println!("{}\n{:?}\n{:?}", out.len(), out, output);
                    output[..out.len()].copy_from_slice(out.as_slice());
                    Ok(out.len())
                }

                fn decode_len(len: usize) -> Result<usize> {
                    Ok(encoding::calc_decoded_size($encoding.chars().count(), len))
                }
            }
        )*
    };
}

pub(crate) trait BaseCodec {
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Encode with the given byte slice.
    fn encode<I: AsRef<[u8]>>(input: I) -> String;

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Decode with the given string.
    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>>;

    /// Encode with the given byte slice to a mutable slice.
    fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]);

    /// Returns the encoded length of an input of length `len`
    fn encode_len(len: usize) -> usize;

    /// Encode with the given byte slice to a mutable slice.
    fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize>;

    /// Returns the decoded length of an input of length `len`
    fn decode_len(len: usize) -> Result<usize>;
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

#[cfg(feature = "alloc")]
derive_base_x! {
    /// Base10 (alphabet: 0123456789).
    Base10, encoding::BASE10;
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    Base58Flickr, encoding::BASE58_FLICKR;
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    Base58Btc, encoding::BASE58_BITCOIN;
}

#[cfg(feature = "alloc")]
/// Base36, [0-9a-z] no padding (alphabet: abcdefghijklmnopqrstuvwxyz0123456789).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base36Lower;

#[cfg(feature = "alloc")]
impl BaseCodec for Base36Lower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE36_LOWER, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        // The input is case in-sensitive, hence lowercase it
        let lowercased = input.as_ref().to_ascii_lowercase();
        Ok(base_x::decode(encoding::BASE36_LOWER, &lowercased)?)
    }

    fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]) {
        let out = base_x::encode(encoding::BASE36_LOWER, input.as_ref());
        output[..out.len()].copy_from_slice(out.as_bytes());
    }

    fn encode_len(len: usize) -> usize {
        encoding::calc_encoded_size(encoding::BASE36_LOWER.chars().count(), len)
    }

    fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize> {
        let out = base_x::decode(encoding::BASE36_LOWER, input.as_ref())?;
        output[..out.len()].copy_from_slice(out.as_slice());
        Ok(out.len())
    }

    fn decode_len(len: usize) -> Result<usize> {
        Ok(encoding::calc_decoded_size(
            encoding::BASE36_LOWER.chars().count(),
            len,
        ))
    }
}

#[cfg(feature = "alloc")]
/// Base36, [0-9A-Z] no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base36Upper;

#[cfg(feature = "alloc")]
impl BaseCodec for Base36Upper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE36_UPPER, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        // The input is case insensitive, hence uppercase it
        let uppercased = input.as_ref().to_ascii_uppercase();
        Ok(base_x::decode(encoding::BASE36_UPPER, &uppercased)?)
    }

    fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]) {
        let out = base_x::encode(encoding::BASE36_UPPER, input.as_ref());
        output[..out.len()].copy_from_slice(out.as_bytes());
    }

    fn encode_len(len: usize) -> usize {
        encoding::calc_encoded_size(encoding::BASE36_UPPER.chars().count(), len)
    }

    fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize> {
        let out = base_x::decode(encoding::BASE36_UPPER, input.as_ref())?;
        output[..out.len()].copy_from_slice(out.as_slice());
        Ok(out.len())
    }

    fn decode_len(len: usize) -> Result<usize> {
        Ok(encoding::calc_decoded_size(
            encoding::BASE36_UPPER.chars().count(),
            len,
        ))
    }
}

/// Identity, 8-bit binary (encoder and decoder keeps data unmodified).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Identity;

impl BaseCodec for Identity {
    #[cfg(feature = "alloc")]
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        String::from_utf8(input.as_ref().to_vec()).expect("input must be valid UTF-8 bytes")
    }

    #[cfg(feature = "alloc")]
    fn decode<I: AsRef<str>>(input: I) -> Result<Vec<u8>> {
        Ok(input.as_ref().as_bytes().to_vec())
    }

    fn encode_mut<I: AsRef<[u8]>>(input: I, output: &mut [u8]) {
        output.copy_from_slice(input.as_ref());
    }

    fn encode_len(len: usize) -> usize {
        len
    }

    fn decode_mut<I: AsRef<str>>(input: I, output: &mut [u8]) -> Result<usize> {
        output.copy_from_slice(input.as_ref().as_bytes());
        Ok(input.as_ref().len())
    }

    fn decode_len(len: usize) -> Result<usize> {
        Ok(len)
    }
}
