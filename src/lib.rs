/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

extern crate base_x;

/// Error types
#[derive(Debug)]
pub enum MultibaseError {
    UnsupportedBase,
    UnkownBase,
}

/// Result type
pub type MultibaseResult = Result<String, MultibaseError>;

/// List of supported bases.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Base {
    /// unary tends to be 11111
    Base1,
    /// binary has 1 and 0
    Base2,
    /// highest char in octal
    Base8,
    /// highest char in decimal
    Base10,
    /// highest char in hex
    Base16,
    Base16Upper,
    /// rfc4648 no padding - highest char
    Base32hex,
    Base32hexUpper,
    /// rfc4648 with padding
    Base32hexpad,
    Base32hexpadUpper,
    /// rfc4648 no padding
    Base32,
    Base32Upper,
    /// rfc4648 with padding
    Base32pad,
    Base32padUpper,
    /// z-base-32 - used by Tahoe-LAFS - highest letter
    Base32z,
    /// highest letter
    Base58flickr,
    /// highest letter
    Base58btc,
    /// rfc4648 no padding
    Base64,
    /// rfc4648 with padding - MIME encoding
    Base64pad,
    /// rfc4648 no padding
    Base64url,
    /// rfc4648 with padding
    Base64urlpad,
}

impl Base {
    /// Get the base code.
    pub fn code(&self) -> &str {
        use Base::*;

        match *self {
            Base1 => "1",
            Base2 => "0",
            Base8 => "7",
            Base10 => "9",
            Base16 => "f",
            Base16Upper => "F",
            Base32hex => "v",
            Base32hexUpper => "V",
            Base32hexpad => "t",
            Base32hexpadUpper => "T",
            Base32 => "b",
            Base32Upper => "B",
            Base32pad => "c",
            Base32padUpper => "C",
            Base32z => "h",
            Base58flickr => "Z",
            Base58btc => "z",
            Base64 => "m",
            Base64pad => "M",
            Base64url => "u",
            Base64urlpad => "U",
        }
    }

    /// Get the matching alphabet.
    pub fn alphabet(&self) -> Result<&str, MultibaseError> {
        use Base::*;

        match *self {
            Base1 => Ok("1"),
            Base2 => Ok("01"),
            Base8 => Ok("01234567"),
            Base10 => Ok("0123456789"),
            Base16 => Ok("0123456789abcdef"),
            Base16Upper => Ok("0123456789ABCDEF"),
            Base32hex => Ok("0123456789abcdefghijklmnopqrstuv"),
            Base32hexUpper => Ok("0123456789ABCDEFGHIJKLMNOPQRSTUV"),
            Base32hexpad => Err(MultibaseError::UnsupportedBase),
            Base32hexpadUpper => Err(MultibaseError::UnsupportedBase),
            Base32 => Ok("abcdefghijklmnopqrstuvwxyz234567"),
            Base32Upper => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"),
            Base32pad => Err(MultibaseError::UnsupportedBase),
            Base32padUpper => Err(MultibaseError::UnsupportedBase),
            Base32z => Ok("ybndrfg8ejkmcpqxot1uwisza345h769"),
            Base58flickr => Ok("123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"),
            Base58btc => Ok("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
            Base64 => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"),
            Base64pad => Err(MultibaseError::UnsupportedBase),
            Base64url => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            Base64urlpad => Err(MultibaseError::UnsupportedBase),
        }
    }

    /// Convert a code to a base.
    pub fn from_code(code: &str) -> Result<Base, MultibaseError> {
        use Base::*;

        match code {
            "1" => Ok(Base1),
            "0" => Ok(Base2),
            "7" => Ok(Base8),
            "9" => Ok(Base10),
            "Ff" => Ok(Base16),
            "Vv" => Ok(Base32hex),
            "Tt" => Ok(Base32hexpad),
            "Bb" => Ok(Base32),
            "Cc" => Ok(Base32pad),
            "h" => Ok(Base32z),
            "Z" => Ok(Base58flickr),
            "z" => Ok(Base58btc),
            "m" => Ok(Base64),
            "M" => Ok(Base64pad),
            "u" => Ok(Base64url),
            "U" => Ok(Base64urlpad),
            _ => Err(MultibaseError::UnkownBase),
        }
    }
}

pub fn encode(base: Base, data: &str) -> MultibaseResult {
    match base.alphabet() {
        Ok(alphabet) => {
            let chars: Vec<i16> = data.encode_utf16()
                .map((|u| u as i16))
                .collect();
            Ok(base.code().to_string() + &base_x::encode(alphabet, chars))
        }
        Err(why) => Err(why),
    }
}


#[cfg(test)]
mod tests {
    use ::encode;
    use Base;

    #[test]
    fn test_bases_code() {
        assert_eq!(Base::Base1.code(), "1");
        assert_eq!(Base::Base64urlpad.code(), "U");
    }

    #[test]
    fn test_bases_from_code() {
        assert_eq!(Base::from_code("1").unwrap(), Base::Base1);
        assert_eq!(Base::from_code("U").unwrap(), Base::Base64urlpad);
    }

    #[test]
    fn test_encode() {
        let id = "Decentralize everything!!";

        assert_eq!(encode(Base::Base16, &id).unwrap(),
                   "f446563656e7472616c697a652065766572797468696e672121");
    }
}
