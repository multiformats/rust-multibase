use crate::Base;

/// Trait implemented for byte-array-like types.
pub trait Encodable {
    /// Encode with the given base
    fn encode(&self, base: Base) -> String;
}

impl Encodable for [u8] {
    #[inline]
    fn encode(&self, base: Base) -> String {
        let mut encoded = match base {
            Base::Base64 => {
                base64::encode_config(self, base64::STANDARD_NO_PAD)
            }
            Base::Base64pad => {
                base64::encode_config(self, base64::STANDARD)
            }
            Base::Base64url => {
                base64::encode_config(self, base64::URL_SAFE_NO_PAD)
            }
            Base::Base64urlpad => {
                base64::encode_config(self, base64::URL_SAFE)
            }
            _ => {
                let alphabet = base.alphabet();
                base_x::encode(alphabet, self)
            }
        };
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
