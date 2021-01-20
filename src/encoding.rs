use data_encoding::Encoding;
use data_encoding_macro::new_encoding;

// At a later point (probably when floating point arithmatic in const fn is stable) the below functions can be const

#[cfg(feature = "alloc")]
/// math comes from here https://github.com/bitcoin/bitcoin/blob/f1e2f2a85962c1664e4e55471061af0eaa798d40/src/base58.cpp#L94
pub(crate) fn gen_encoded_size(base: usize, input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (f64::log10(256.0) / f64::log10(base as f64))) as usize + 1
}

#[cfg(feature = "alloc")]
/// math comes from here https://github.com/bitcoin/bitcoin/blob/f1e2f2a85962c1664e4e55471061af0eaa798d40/src/base58.cpp#L48
pub(crate) fn gen_decoded_size(base: usize, input_byte_size: usize) -> usize {
    f64::ceil(input_byte_size as f64 * (f64::log10(base as f64) / f64::log10(256.0))) as usize + 1 // ceil may be excessive but its on the safer side and it works
}

// Base2 (alphabet: 01)
pub const BASE2: Encoding = new_encoding! {
    symbols: "01",
};

// Base8 (alphabet: 01234567)
pub const BASE8: Encoding = new_encoding! {
    symbols: "01234567",
};

#[cfg(feature = "alloc")]
/// Base10 (alphabet: 0123456789)
pub const BASE10: (&str, usize) = ("0123456789", 10);

// Base16 lower hexadecimal (alphabet: 0123456789abcdef)
pub const BASE16_LOWER: Encoding = data_encoding::HEXLOWER_PERMISSIVE;

// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
pub const BASE16_UPPER: Encoding = data_encoding::HEXUPPER_PERMISSIVE;

// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    translate_to: "abcdefghijklmnopqrstuvwxyz",
};

// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_NOPAD_UPPER: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    translate_from: "abcdefghijklmnopqrstuvwxyz",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
};

// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_PAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    translate_to: "abcdefghijklmnopqrstuvwxyz",
    padding: '=',
};

// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_PAD_UPPER: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    translate_from: "abcdefghijklmnopqrstuvwxyz",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    padding: '=',
};

// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUV",
    translate_to: "abcdefghijklmnopqrstuv",
};

// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_NOPAD_UPPER: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
    translate_from: "abcdefghijklmnopqrstuv",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUV",
};

// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_PAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUV",
    translate_to: "abcdefghijklmnopqrstuv",
    padding: '=',
};

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_PAD_UPPER: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
    translate_from: "abcdefghijklmnopqrstuv",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUV",
    padding: '=',
};

// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
pub const BASE32Z: Encoding = new_encoding! {
    symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
};

#[cfg(feature = "alloc")]
/// Base36, [0-9a-z] no padding (alphabet: 0123456789abcdefghijklmnopqrstuvwxyz).
pub const BASE36_LOWER: (&str, usize) = ("0123456789abcdefghijklmnopqrstuvwxyz", 36);

#[cfg(feature = "alloc")]
/// Base36, [0-9A-Z] no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).
pub const BASE36_UPPER: (&str, usize) = ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", 36);

#[cfg(feature = "alloc")]
// Base58 Flickr's alphabet for creating short urls from photo ids.
pub const BASE58_FLICKR: (&str, usize) = (
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ",
    58,
);

#[cfg(feature = "alloc")]
// Base58 Bitcoin's alphabet as defined in their Base58Check encoding.
pub const BASE58_BITCOIN: (&str, usize) = (
    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
    58,
);

// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_NOPAD: Encoding = data_encoding::BASE64_NOPAD;

// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_PAD: Encoding = data_encoding::BASE64;

// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_NOPAD: Encoding = data_encoding::BASE64URL_NOPAD;

// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_PAD: Encoding = data_encoding::BASE64URL;
