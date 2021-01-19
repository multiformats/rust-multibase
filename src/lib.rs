//! # multibase
//!
//! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.
//! 
//! Usable without a global allocator for all encodings except those backed by [base-x](https://github.com/OrKoN/base-x-rs)

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

mod base;
mod encoding;
mod error;
mod impls;

pub use self::base::Base;
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
#[cfg(feature = "alloc")]
pub fn decode<T: AsRef<str>>(input: T) -> Result<(Base, Vec<u8>)> {
    let input = input.as_ref();
    let code = input.chars().next().ok_or(Error::InvalidBaseString)?;
    let base = Base::from_code(code)?;
    let decoded = base.decode(&input[code.len_utf8()..])?;
    Ok((base, decoded))
}

/// Decode the base string into a mutable slice.
///
/// NOTE: while you _can_ use this method for non-byte aligned encodings, the `decode` method does exactly the same thing in a more ergonomic way.
/// This is designed to be used when there is no global allocator.
/// 
/// # Examples
///
/// ```
/// use multibase::{Base, decode_mut};
/// 
/// let input = "MaGVsbG8gd29ybGQ=";
/// let mut buffer = &mut [0u8; 255];
/// 
/// let code = input.chars().next().unwrap();
/// let base = Base::from_code(code).unwrap();
/// 
/// let output = &mut buffer[0 .. base.decode_len(input.len()).unwrap()];
/// decode_mut(base, input, output);
/// assert_eq!(
///     (base, core::str::from_utf8(output).unwrap().trim_end_matches('\u{0}')),
///     (Base::Base64Pad, "hello world")
/// );
/// ```
pub fn decode_mut<T: AsRef<str>>(base: Base, input: T, output: &mut [u8]) -> Result<()> {
    let input = input.as_ref();
    base.decode_mut(&input[base.code().len_utf8()..], output)?;
    Ok(())
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
#[cfg(feature = "alloc")]
pub fn encode<T: AsRef<[u8]>>(base: Base, input: T) -> String {
    let input = input.as_ref();
    let mut encoded = base.encode(input.as_ref());
    encoded.insert(0, base.code());
    encoded
}

/// Encode the given byte slice to a mutable slice.
///
/// NOTE: while you _can_ use this method for non-byte aligned encodings, the `encode` method does exactly the same thing in a more ergonomic way.
/// This is designed to be used when there is no global allocator.
/// 
/// # Examples
///
/// ```
/// use multibase::{Base, encode_mut};
/// 
/// let input = "hello world";
/// let mut buffer = &mut [0u8; 255];
/// 
/// let base = Base::Base64Pad;
/// 
/// let output = &mut buffer[0 .. base.encode_len(input.len())];
/// encode_mut(base, input, output);
/// assert_eq!(
///     core::str::from_utf8(output).unwrap(),
///     "MaGVsbG8gd29ybGQ="
/// );
/// ```
pub fn encode_mut<T: AsRef<str>>(base: Base, input: T, output: &mut [u8]) -> Result<()> {
    base.code().encode_utf8(output);
    base.encode_mut(input.as_ref(), &mut output[base.code().len_utf8()..]);
    Ok(())
}