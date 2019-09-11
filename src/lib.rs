/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.
mod base;
mod error;

pub use base::Base;
pub use error::{Error, Result};

/// Decode the string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, decode};
///
/// assert_eq!(decode("zCn8eVZg").unwrap(),
///            (Base::Base58btc, b"hello".to_vec()));
/// ```
pub fn decode<T: AsRef<str>>(input: T) -> Result<(Base, Vec<u8>)> {
    let input = input.as_ref();
    let code = input.chars().next().ok_or(Error::InvalidBaseString)?;
    let base = Base::from_code(code)?;
    let content = &input[code.len_utf8()..];
    let decoded = base.decode(content)?;
    Ok((base, decoded))
}

/// Encode with the given string
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode};
///
/// assert_eq!(encode(Base::Base58btc, b"hello"),
///            "zCn8eVZg");
/// ```
pub fn encode<T: AsRef<[u8]>>(base: Base, input: T) -> String {
    let input = input.as_ref();
    let mut encoded = base.encode(input.as_ref());
    encoded.insert(0, base.code());
    encoded
}
