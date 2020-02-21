use data_encoding::{Encoding, Specification};
use lazy_static::lazy_static;

fn make_encoding(symbols: &str, padding: Option<char>) -> Encoding {
    let mut spec = Specification::new();
    spec.symbols.push_str(symbols);
    spec.padding = padding;
    spec.encoding().unwrap()
}

// Base2 (alphabet: 01)
lazy_static! {
    pub static ref BASE2: Encoding = make_encoding("01", None);
}

// Base8 (alphabet: 01234567)
lazy_static! {
    pub static ref BASE8: Encoding = make_encoding("01234567", None);
}

/// Base10 (alphabet: 0123456789)
pub const BASE10: &str = "0123456789";

// Base16 lower hexadecimal (alphabet: 0123456789abcdef)
pub const BASE16_LOWER: Encoding = data_encoding::HEXLOWER;

// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
pub const BASE16_UPPER: Encoding = data_encoding::HEXUPPER;

// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
lazy_static! {
    pub static ref BASE32_NOPAD_LOWER: Encoding =
        make_encoding("abcdefghijklmnopqrstuvwxyz234567", None);
}

// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_NOPAD_UPPER: Encoding = data_encoding::BASE32_NOPAD;

// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
lazy_static! {
    pub static ref BASE32_PAD_LOWER: Encoding =
        make_encoding("abcdefghijklmnopqrstuvwxyz234567", Some('='));
}

// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_PAD_UPPER: Encoding = data_encoding::BASE32;

// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
lazy_static! {
    pub static ref BASE32HEX_NOPAD_LOWER: Encoding =
        make_encoding("0123456789abcdefghijklmnopqrstuv", None);
}

// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_NOPAD_UPPER: Encoding = data_encoding::BASE32HEX_NOPAD;

// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
lazy_static! {
    pub static ref BASE32HEX_PAD_LOWER: Encoding =
        make_encoding("0123456789abcdefghijklmnopqrstuv", Some('='));
}

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_PAD_UPPER: Encoding = data_encoding::BASE32HEX;

// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
lazy_static! {
    pub static ref BASE32Z: Encoding = make_encoding("ybndrfg8ejkmcpqxot1uwisza345h769", None);
}

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
