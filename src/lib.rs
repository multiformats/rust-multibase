/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

mod base;
mod error;
mod decodable;
mod encodable;

pub use decodable::Decodable;
pub use encodable::Encodable;

pub use base::Base;
pub use Base::*;
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
    #[inline]
pub fn decode<T: Decodable>(data: T) -> Result<(Base, Vec<u8>)> {
    data.decode()
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
pub fn encode<T: Encodable>(base: Base, data: T) -> String {
    data.encode(base)
}
