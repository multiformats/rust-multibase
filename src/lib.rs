//! # multibase
//!
//! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

mod base;
mod encoding;
mod error;
mod impls;

pub use self::base::{Base, StackBase};
pub use self::error::{Error, Result};

/// Decode the base string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, decode};
///
/// assert_eq!(
///     decode("zCn8eVZg").unwrap(),
///     (Base::Base58Btc, b"hello".to_vec())
/// );
/// ```
pub fn decode<T: AsRef<str>>(input: T) -> Result<(Base, Vec<u8>)> {
    let input = input.as_ref();
    let code = input.chars().next().ok_or(Error::InvalidBaseString)?;
    let base = Base::from_code(code)?;
    let decoded = base.decode(&input[code.len_utf8()..])?;
    Ok((base, decoded))
}

/// Encode with the given byte slice to base string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode};
///
/// assert_eq!(encode(Base::Base58Btc, b"hello"), "zCn8eVZg");
/// ```
pub fn encode<T: AsRef<[u8]>>(base: Base, input: T) -> String {
    let input = input.as_ref();
    let mut encoded = base.encode(input);
    encoded.insert(0, base.code());
    encoded
}

/// Encode with the given byte slice to *stack allocated* base string.
///
/// # Examples
///
/// ```
/// use multibase::{StackBase, encode_arr};
///
/// assert_eq!(encode_arr::<32>(StackBase::Base58BtcS, b"hello").unwrap(), "zCn8eVZg");
/// ```
pub fn encode_arr<const S: usize>(
    base: StackBase<0, S>,
    input: &[u8],
) -> Result<heapless::String<S>> {
    let mut out = base.encode(input)?;
    // encode() leaves an open byte in the begining (for stack implementations)
    // SAFETY: this trusts that and all (implemented) multibase codes are ascii
    unsafe { out.as_mut_vec()[0] = base.code() as u8 };
    Ok(out)
}

/// Decode the given byte slice to *stack allocated* output buffer.
///
/// # Examples
///
/// ```
/// use multibase::{StackBase, decode_arr};
///
/// assert_eq!(decode_arr::<32>("zCn8eVZg").unwrap().1.as_slice(), "hello".as_bytes());
/// ```
pub fn decode_arr<const S: usize>(input: &str) -> Result<(StackBase<S, 0>, heapless::Vec<u8, S>)> {
    let code = input.chars().next().ok_or(Error::InvalidBaseString)?;
    let base = StackBase::from_code(code)?;
    let decoded = base.decode(&input[code.len_utf8()..])?;
    Ok((base, decoded))
}

/// generates encoded size of a given base and input length, useful for determining max encoded size
///
/// given as `smol_base_x::encoded_size + 1`
pub fn base_x_encoded_size(base: usize, bytes_len: usize) -> usize {
    use smol_base_x::util::encoded_size;
    encoded_size(base, bytes_len) + 1
}

/// generates decoded size of a given base and input byte length, useful for determining decoded size
///
/// given as `smol_base_x::decoded_size - 1`
pub fn base_x_decoded_size(base: usize, bytes_len: usize) -> usize {
    use smol_base_x::util::decoded_size;
    decoded_size(base, bytes_len) - 1
}
