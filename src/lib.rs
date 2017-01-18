/// ! # multibase
/// !
/// ! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

/// List of supported bases.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Bases {
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
    /// rfc4648 no padding - highest char
    Base32hex,
    /// rfc4648 with padding
    Base32hexpad,
    /// rfc4648 no padding
    Base32,
    /// rfc4648 with padding
    Base32pad,
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

impl Bases {
    /// Get the base code.
    pub fn code(&self) -> &str {
        use Bases::*;

        match *self {
            Base1 => "1",
            Base2 => "0",
            Base8 => "7",
            Base10 => "9",
            Base16 => "Ff",
            Base32hex => "Vv",
            Base32hexpad => "Tt",
            Base32 => "Bb",
            Base32pad => "Cc",
            Base32z => "h",
            Base58flickr => "Z",
            Base58btc => "z",
            Base64 => "m",
            Base64pad => "M",
            Base64url => "u",
            Base64urlpad => "U",
        }
    }

    /// Convert a code to a base.
    pub fn from_code(code: &str) -> Option<Bases> {
        use Bases::*;

        match code {
            "1" => Some(Base1),
            "0" => Some(Base2),
            "7" => Some(Base8),
            "9" => Some(Base10),
            "Ff" => Some(Base16),
            "Vv" => Some(Base32hex),
            "Tt" => Some(Base32hexpad),
            "Bb" => Some(Base32),
            "Cc" => Some(Base32pad),
            "h" => Some(Base32z),
            "Z" => Some(Base58flickr),
            "z" => Some(Base58btc),
            "m" => Some(Base64),
            "M" => Some(Base64pad),
            "u" => Some(Base64url),
            "U" => Some(Base64urlpad),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use Bases;

    #[test]
    fn bases_code() {
        assert_eq!(
            Bases::Base1.code(),
            "1"
        );
        assert_eq!(
            Bases::Base64urlpad.code(),
            "U"
        );
    }

    #[test]
    fn bases_from_code() {
        assert_eq!(
            Bases::from_code("1").unwrap(),
            Bases::Base1
        );
        assert_eq!(
            Bases::from_code("U").unwrap(),
            Bases::Base64urlpad
        );
    }
}
