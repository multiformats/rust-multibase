use crate::{Base, Error, Result};

/// Trait implemented for string-like types.
pub trait Decodable {
    /// Decode to base and raw binary data
    fn decode(&self) -> Result<(Base, Vec<u8>)>;
}

impl Decodable for str {
    fn decode(&self) -> Result<(Base, Vec<u8>)> {
        let code = self.chars().next().ok_or(Error::InvalidBaseString)?;
        let base = Base::from_code(code)?;
        let content = &self[code.len_utf8()..];
        let decoded = match base {
            Base::Base64 => {
                base64::decode_config(content, base64::STANDARD_NO_PAD)?
            }
            Base::Base64pad => {
                base64::decode_config(content, base64::STANDARD)?
            }
            Base::Base64url => {
                base64::decode_config(content, base64::URL_SAFE_NO_PAD)?
            }
            Base::Base64urlpad => {
                base64::decode_config(content, base64::URL_SAFE)?
            }
            _ => {
                let alphabet = base.alphabet();
                base_x::decode(alphabet, content)?
            }
        };
        Ok((base, decoded))
     }
}

impl<'a, D: AsRef<str>> Decodable for D {
    #[inline]
    fn decode(&self) -> Result<(Base, Vec<u8>)> {
        self.as_ref().decode()
    }
}
