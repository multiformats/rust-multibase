use crate::{encoding, Error};
use crate::error::Result;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::String, vec::Vec};

macro_rules! derive_base_encoding {
    ( $(#[$doc:meta] $code:literal => $type:ident, $encoding:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl CodecCode for $type {
                const CODE: char = $code;
            }

            impl BaseCodec<String, Vec<u8>> for $type {
                fn encode(input: impl AsRef<[u8]>) -> Result<String> {
                    Ok($encoding.encode(input.as_ref()))
                }

                fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
                    Ok($encoding.decode(input.as_ref().as_bytes())?)
                }
            }

            impl<const S: usize, const B: usize> BaseCodec<heapless::String<S>, heapless::Vec<u8, B>> for $type {
                fn encode(input: impl AsRef<[u8]>) -> Result<heapless::String<S>> {
                    let input = input.as_ref();
                    let mut s = heapless::String::<S>::default();
            
                    // SAFETY: trait Base should only contain ascii chars (otherwise would be a borken base implementation)
                    unsafe {
                        let vec = s.as_mut_vec();
                        // resize is a safe operation
                        let len = $encoding.encode_len(input.len());
            
                        vec.resize(len, 0)
                            .map_err(|_| Error::ContainerTooSmall)?;
                        // skips first byte to leave room for multibase code
                        $encoding.encode_mut(input, &mut vec[1..]);
                    }
                    Ok(s)
                }
            
                fn decode(input: impl AsRef<str>) -> Result<heapless::Vec<u8, B>> {
                    let input = input.as_ref();
                    let mut buf = heapless::Vec::<u8, B>::default();
                    let len = $encoding.decode_len(input.len())?;
                    buf.resize(len, 0)
                        .map_err(|_| Error::ContainerTooSmall)?;
                    $encoding.decode_mut(input.as_bytes(), &mut buf).unwrap();
                    Ok(buf)
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

            impl CodecCode for $type {
                const CODE: char = $code;
            }

            impl BaseCodec<String, Vec<u8>> for $type {
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

pub(crate) trait CodecCode {
    const CODE: char;
}

/// Trait codecs use for encoding and decoding, generic over their output continer
/// output container of encode must have the ability to prepend one byte for the codec code
pub(crate) trait BaseCodec<En, De>: CodecCode {
    /// Encode with the given byte slice.
    fn encode(input: impl AsRef<[u8]>) -> Result<En>;
    /// Decode with the given string (as slice).
    fn decode(input: impl AsRef<str>) -> Result<De>;
}

#[cfg(feature = "alloc")]
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

#[cfg(feature = "alloc")]
derive_base_x! {
    /// Base10 (alphabet: 0123456789).
    '9' => Base10, encoding::BASE10;
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    'Z' => Base58Flickr, encoding::BASE58_FLICKR;
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    'z' => Base58Btc, encoding::BASE58_BITCOIN;
}

#[cfg(feature = "alloc")]
pub(crate) mod alloc {
    use crate::encoding;

    /// Identity, 8-bit binary (encoder and decoder keeps data unmodified).
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub(crate) struct Identity;

    impl CodecCode for Identity {
        const CODE: char = '\x00';
    }

    impl BaseCodec<String, Vec<u8>> for Identity {
        fn encode(input: impl AsRef<[u8]>) -> Result<String> {
            String::from_utf8(input.as_ref().to_vec()).map_err(|_| crate::Error::InvalidBaseString)
        }

        fn decode(input: impl AsRef<str>) -> Result<Vec<u8>> {
            Ok(input.as_ref().as_bytes().to_vec())
        }
    }

    use super::{BaseCodec, CodecCode, Result};
    /// Base36, [0-9a-z] no padding (alphabet: abcdefghijklmnopqrstuvwxyz0123456789).
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub(crate) struct Base36Lower;

    impl CodecCode for Base36Lower {
        const CODE: char = 'k';
    }

    impl BaseCodec<String, Vec<u8>> for Base36Lower {
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

    impl CodecCode for Base36Upper {
        const CODE: char = 'K';
    }

    impl BaseCodec<String, Vec<u8>> for Base36Upper {
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

pub mod smol {

    // TODO not unwrap

    use heapless::{String, Vec};
    use smol_base_x::Base;

    use crate::Error;

    use super::BaseCodec;
    use super::CodecCode;
    use super::Result;

    // it sucks a lot that I can't
    // impl<const S: usize, const B: usize, T, const BASE: usize> BaseCodec<String<S>, Vec<u8, B>> for T where
    //     T: Base<BASE> + CodecCode

    macro_rules! derive_smol_base_x {
        ( $(#[$doc:meta] $code:literal => $base:literal:$type:ident, $encoding:expr;)* ) => {
            $(
                #[$doc]
                #[derive(PartialEq, Eq, Clone, Copy, Debug)]
                pub(crate) struct $type;

                impl Base<$base> for $type {
                    const ALPHABET: [u8; $base] = $encoding;
                }

                impl CodecCode for $type {
                    const CODE: char = $code;
                }

                impl<const S: usize, const B: usize> BaseCodec<String<S>, Vec<u8, B>> for $type {
                    fn encode(input: impl AsRef<[u8]>) -> Result<String<S>> {
                        let input = input.as_ref();
                        let mut s = String::<S>::default();

                        // SAFETY: trait Base should only contain ascii chars (otherwise would be a borken base implementation)
                        unsafe {
                            let vec = s.as_mut_vec();
                            // resize is a safe operation
                            let len = crate::base_x_encoded_size(Self::BASE, input.len());
                            vec.resize(len, 0)
                                .map_err(|_| Error::ContainerTooSmall)?;
                            // skips first byte to leave room for multibase code
                            $type::encode_mut(input, &mut vec[1..]).unwrap();
                        }
                        Ok(s)
                    }

                    fn decode(input: impl AsRef<str>) -> Result<Vec<u8, B>> {
                        let input = input.as_ref();
                        let mut buf = Vec::<u8, B>::default();
                        let len = crate::base_x_decoded_size(Self::BASE, input.len());
                        buf.resize(len, 0)
                            .map_err(|_| Error::ContainerTooSmall)?;
                        $type::decode_mut(input, &mut buf).unwrap();
                        Ok(buf)
                    }
                }
            )*
        };
    }

    derive_smol_base_x! {
        /// Base10 (alphabet: 0123456789).
        '9' => 10:Base10S, *b"0123456789";
        /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
        'z' => 58:Base58BtcS, *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
        'Z' => 58:Base58FlickrS, *b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    }
}
