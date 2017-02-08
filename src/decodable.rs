use {base_x, Base, Error, Result};

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
        let alphabet = base.alphabet();
        let decoded = base_x::decode(alphabet, content)?;
        Ok((base, decoded))
     }
}

impl<'a, D: AsRef<str>> Decodable for D {
    #[inline]
    fn decode(&self) -> Result<(Base, Vec<u8>)> {
        self.as_ref().decode()
    }
}
