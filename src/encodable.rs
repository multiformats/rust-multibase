use {base_x, Base};

/// Trait implemented for byte-array-like types.
pub trait Encodable {
    /// Encode with the given base
    fn encode(&self, base: Base) -> String;
}

impl Encodable for [u8] {
    #[inline]
    fn encode(&self, base: Base) -> String {
        let alphabet = base.alphabet();

        let mut encoded = base_x::encode(alphabet, self);
        encoded.insert(0, base.code());
        encoded
    }
}

impl<'a, E: AsRef<[u8]>> Encodable for E {
    #[inline]
    fn encode(&self, base: Base) -> String {
        self.as_ref().encode(base)
    }
}
