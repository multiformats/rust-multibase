use data_encoding::Encoding;
use data_encoding_macro::{internal_new_encoding, new_encoding};

// Base2 (alphabet: 01)
pub const BASE2: Encoding = new_encoding! {
    symbols: "01",
};

// Base8 (alphabet: 01234567)
pub const BASE8: Encoding = new_encoding! {
    symbols: "01234567",
};

/// Base10 (alphabet: 0123456789)
pub const BASE10: &str = "0123456789";

// Base16 lower hexadecimal (alphabet: 0123456789abcdef)
pub const BASE16_LOWER: Encoding = data_encoding::HEXLOWER;

// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
pub const BASE16_UPPER: Encoding = data_encoding::HEXUPPER;

// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
};

// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_NOPAD_UPPER: Encoding = data_encoding::BASE32_NOPAD;

// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_PAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    padding: '=',
};

// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_PAD_UPPER: Encoding = data_encoding::BASE32;

// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
};

// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_NOPAD_UPPER: Encoding = data_encoding::BASE32HEX_NOPAD;

// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_PAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    padding: '=',
};

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_PAD_UPPER: Encoding = data_encoding::BASE32HEX;

// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
pub const BASE32Z: Encoding = new_encoding! {
    symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
};

// Base58 Flickr's alphabet for creating short urls from photo ids.
pub const BASE58_FLICKR: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

// Base58 Bitcoin's alphabet as defined in their Base58Check encoding.
pub const BASE58_BITCOIN: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_NOPAD: Encoding = data_encoding::BASE64_NOPAD;

// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_PAD: Encoding = data_encoding::BASE64;

// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_NOPAD: Encoding = data_encoding::BASE64URL_NOPAD;

// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_PAD: Encoding = data_encoding::BASE64URL;
