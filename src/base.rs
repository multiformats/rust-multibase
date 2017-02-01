use std::{error, fmt};

#[derive(Debug)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode the given data")
    }
}

impl error::Error for DecodeError {
    fn description(&self) -> &str {
        "Can not decode the provided data"
    }
}

/// Encode an input vector using the given alphabet.
pub fn encode(alphabet: &[u8], input: &[u8]) -> Vec<u8> {
    if input.len() == 0 {
        return Vec::new();
    }

    let base = alphabet.len() as u16;

    let mut digits: Vec<u16> = Vec::with_capacity(input.len());

    digits.push(0);

    for c in input {
        let mut j = 0;
        let mut carry = *c as u16;

        while j < digits.len() {
            carry = carry + (digits[j] << 8);
            digits[j] = carry % base;
            carry /= base;
            j += 1;
        }

        while carry > 0 {
            digits.push(carry % base);
            carry /= base;
        }
    }

    let leaders = input
        .iter()
        .take(input.len() - 1)
        .take_while(|i| **i == 0)
        .map(|_| 0);

    digits.extend(leaders);

    digits.iter().rev().map(|digit| alphabet[*digit as usize]).collect()
}

/// Decode an input vector using the given alphabet.
pub fn decode(alphabet: &[u8], input: &[u8]) -> Result<Vec<u8>, DecodeError> {
    if input.len() == 0 {
        return Ok(Vec::new());
    }

    let base = alphabet.len() as u16;
    let leader = alphabet.get(0).ok_or(DecodeError)?;

    // 0xFF is not a valid ASCII value, so it can be safely used as
    // invalid marker in the lookup table
    const INVALID: u8 = 0xFF;

    // Ideally this lookup table would be generated on compile time for
    // All the alphabets. That said, this should be pretty darn fast anyway.
    let mut alphabet_lut = [INVALID; 256];

    for (i, byte) in alphabet.iter().enumerate() {
        alphabet_lut[*byte as usize] = i as u8;
    }

    let mut bytes: Vec<u8> = vec![0];

    for c in input {
        let mut carry = match alphabet_lut[*c as usize] {
            INVALID => return Err(DecodeError),
            carry => carry,
        } as u16;

        for byte in bytes.iter_mut() {
            carry += (*byte as u16) * base;
            *byte = carry as u8;
            carry >>= 8;
        }

        while carry > 0 {
            bytes.push(carry as u8);
            carry >>= 8;
        }
    }

    let leaders = input.iter()
        .take(input.len() - 1)
        .take_while(|byte| *byte == leader)
        .map(|_| 0);

    bytes.extend(leaders);
    bytes.reverse();
    Ok(bytes)
}

#[cfg(test)]
mod test {
    const BASE2: &'static [u8] = b"01";
    const BASE16: &'static [u8] = b"0123456789abcdef";
    const BASE58: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    use super::encode;
    use super::decode;

    macro_rules! make_test {
        ($name:ident, $alph:expr, $data:expr, $expect:expr) => {
            #[test]
            fn $name() {
                let data: &[u8] = $data;
                let expect: &[u8] = $expect;

                let encoded = encode($alph, data);
                assert_eq!(encoded, expect, "Encoding is ok");

                let decoded = decode($alph, expect).expect("Decoding must succeed");
                assert_eq!(decoded, data, "Decoding is ok");
            }
        }
    }

    make_test!(base2_a, BASE2, &[0x00,0x0f], b"01111");
    make_test!(base2_b, BASE2, &[0x00,0xff], b"011111111"); // Note the first leading zero byte is compressed into 1 char
    make_test!(base2_c, BASE2, &[0x0f,0xff], b"111111111111");
    make_test!(base2_d, BASE2, &[0xff,0x00,0xff,0x00], b"11111111000000001111111100000000");

    make_test!(base16_a, BASE16, &[0x00,0x00,0x00,0x0f], b"000f");
    make_test!(base16_b, BASE16, &[0x00,0x0f,0xff], b"0fff");
    make_test!(base16_c, BASE16, &[0xff,0xff], b"ffff");

    make_test!(base58_a, BASE58, &[], b"");
    make_test!(base58_b, BASE58, &[0x61], b"2g");
    make_test!(base58_c, BASE58, &[0x62,0x62,0x62], b"a3gV");
    make_test!(base58_d, BASE58, &[0x63,0x63,0x63], b"aPEr");
    make_test!(base58_e, BASE58,
        &[0x73,0x69,0x6d,0x70,0x6c,0x79,0x20,0x61,0x20,0x6c,0x6f,0x6e,0x67,0x20,0x73,0x74,0x72,0x69,0x6e,0x67],
        b"2cFupjhnEsSn59qHXstmK2ffpLv2"
    );
    make_test!(base58_f, BASE58,
        &[0x00,0xeb,0x15,0x23,0x1d,0xfc,0xeb,0x60,0x92,0x58,0x86,0xb6,0x7d,0x06,0x52,0x99,0x92,0x59,0x15,0xae,0xb1,0x72,0xc0,0x66,0x47],
        b"1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"
    );
    make_test!(base58_g, BASE58,
        &[0x51,0x6b,0x6f,0xcd,0x0f],
        b"ABnLTmg"
    );

    // TODO: Add more tests from fixtures
}
