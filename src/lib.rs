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

/// Encoding result type
pub type MultibaseEncodeResult = Result<String, MultibaseError>;

/// Decoding result type
pub type MultibaseDecodeResult = Result<(Base, String), MultibaseError>;

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
            "f" => Ok(Base16),
            "F" => Ok(Base16Upper),
            "v" => Ok(Base32hex),
            "V" => Ok(Base32hexUpper),
            "t" => Ok(Base32hexpad),
            "T" => Ok(Base32hexpadUpper),
            "b" => Ok(Base32),
            "B" => Ok(Base32Upper),
            "c" => Ok(Base32pad),
            "C" => Ok(Base32padUpper),
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

/// Encode a given string with the specified base.
pub fn encode(base: Base, data: &str) -> MultibaseEncodeResult {
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

/// Decode the string.
pub fn decode(data: &str) -> MultibaseDecodeResult {
    let base_char = &data.chars().nth(0).unwrap().to_string();
    println!("base {:}", base_char);
    let base = Base::from_code(
        base_char
    ).unwrap();

    let res: String = base_x::decode(base.alphabet().unwrap(), data)
        .into_iter()
        .map(|u| u.to_string())
        .collect();
    Ok((Base::Base1, res))
}

#[cfg(test)]
mod tests {
    use ::{encode, decode};
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

        assert_eq!(encode(Base::Base58btc, &id).unwrap(),
                   "zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe");

        let id2 = "yes mani !";

        assert_eq!(encode(Base::Base2, &id2).unwrap(),
                   "01111001011001010111001100100000011011010110000101101110011010010010000000100001");
        assert_eq!(encode(Base::Base8, &id2).unwrap(),
                   "7171312714403326055632220041");
        assert_eq!(encode(Base::Base10, &id2).unwrap(),
                   "9573277761329450583662625");
        assert_eq!(encode(Base::Base16, &id2).unwrap(),
                   "f796573206d616e692021");
        assert_eq!(encode(Base::Base32hex, &id2).unwrap(),
                   "vf5in683dc5n6i811");
        assert_eq!(encode(Base::Base32, &id2).unwrap(),
                   "bpfsxgidnmfxgsibb");
        assert_eq!(encode(Base::Base32z, &id2).unwrap(),
                   "hxf1zgedpcfzg1ebb");
        assert_eq!(encode(Base::Base58flickr, &id2).unwrap(),
                   "Z7Pznk19XTTzBtx");
        assert_eq!(encode(Base::Base58btc, &id2).unwrap(),
                   "z7paNL19xttacUY");
    }

    #[test]
    fn test_decode() {
        let id = "Decentralize everything!!";

        assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121").unwrap(),
                   (Base::Base16, id.to_string())
        );

        assert_eq!(decode("zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe").unwrap(),
                   (Base::Base58btc, id.to_string())
        );

        // let id2 = "yes mani !";

        // assert_eq!(encode(Base::Base2, &id2).unwrap(),
        //            "01111001011001010111001100100000011011010110000101101110011010010010000000100001");
        // assert_eq!(encode(Base::Base8, &id2).unwrap(),
        //            "7171312714403326055632220041");
        // assert_eq!(encode(Base::Base10, &id2).unwrap(),
        //            "9573277761329450583662625");
        // assert_eq!(encode(Base::Base16, &id2).unwrap(),
        //            "f796573206d616e692021");
        // assert_eq!(encode(Base::Base32hex, &id2).unwrap(),
        //            "vf5in683dc5n6i811");
        // assert_eq!(encode(Base::Base32, &id2).unwrap(),
        //            "bpfsxgidnmfxgsibb");
        // assert_eq!(encode(Base::Base32z, &id2).unwrap(),
        //            "hxf1zgedpcfzg1ebb");
        // assert_eq!(encode(Base::Base58flickr, &id2).unwrap(),
        //            "Z7Pznk19XTTzBtx");
        // assert_eq!(encode(Base::Base58btc, &id2).unwrap(),
        //            "z7paNL19xttacUY");
    }
}
