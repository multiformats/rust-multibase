use multibase::{decode, encode, Base, Base::*};

#[test]
fn test_bases_code() {
    assert_eq!(Identity.code(), '\x00');
    assert_eq!(Base2.code(), '0');
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code('\x00').unwrap(), Identity);
    assert_eq!(Base::from_code('0').unwrap(), Base2);
}

#[test]
fn test_round_trip() {
    let test_cases: &[&str] = &[
        "helloworld",
        "we all want decentralization",
        "zdj7WfBb6j58iSJuAzDcSZgy2SxFhdpJ4H87uvMpfyN6hRGyH",
    ];

    for case in test_cases {
        let encoded = encode(Base58Btc, case);
        let decoded = decode(encoded).unwrap();
        assert_eq!(decoded, (Base58Btc, case.as_bytes().to_vec()))
    }
}

#[test]
fn test_all() {
    let input = b"Decentralize everything!!!";
    let test_cases = vec![
        (Identity, "\0Decentralize everything!!!"),
        (Base2, "00100010001100101011000110110010101101110011101000111001001100001011011000110100101111010011001010010000001100101011101100110010101110010011110010111010001101000011010010110111001100111001000010010000100100001"),
        (Base8, "72106254331267164344605543227514510062566312711713506415133463441102204"),
        (Base10, "9109908211473026300072608683330054595334719246534349983154512161"),
        (
            Base16Lower,
            "f446563656e7472616c697a652065766572797468696e67212121",
        ),
        (
            Base16Upper,
            "F446563656E7472616C697A652065766572797468696E67212121",
        ),
        (Base32Lower, "birswgzloorzgc3djpjssazlwmvzhs5dinfxgoijbee"),
        (Base32Upper, "BIRSWGZLOORZGC3DJPJSSAZLWMVZHS5DINFXGOIJBEE"),
        (
            Base32PadLower,
            "cirswgzloorzgc3djpjssazlwmvzhs5dinfxgoijbee======",
        ),
        (
            Base32PadUpper,
            "CIRSWGZLOORZGC3DJPJSSAZLWMVZHS5DINFXGOIJBEE======",
        ),
        (Base32HexLower, "v8him6pbeehp62r39f9ii0pbmclp7it38d5n6e89144"),
        (Base32HexUpper, "V8HIM6PBEEHP62R39F9II0PBMCLP7IT38D5N6E89144"),
        (Base32HexPadLower, "t8him6pbeehp62r39f9ii0pbmclp7it38d5n6e89144======"),
        (Base32HexPadUpper, "T8HIM6PBEEHP62R39F9II0PBMCLP7IT38D5N6E89144======"),
        (Base32Z, "het1sg3mqqt3gn5djxj11y3msci3817depfzgqejbrr"),
        (Base58Flickr, "Z36tpRGiQ9Endr7dHahm9xwQdhmoER4emaRVT"),
        (Base58Btc, "z36UQrhJq9fNDS7DiAHM9YXqDHMPfr4EMArvt"),
        (Base64, "mRGVjZW50cmFsaXplIGV2ZXJ5dGhpbmchISE"),
        (Base64Pad, "MRGVjZW50cmFsaXplIGV2ZXJ5dGhpbmchISE="),
        (Base64Url, "uRGVjZW50cmFsaXplIGV2ZXJ5dGhpbmchISE"),
        (Base64UrlPad, "URGVjZW50cmFsaXplIGV2ZXJ5dGhpbmchISE="),
    ];

    for (base, output) in test_cases {
        assert_eq!(encode(base, input), output);
        assert_eq!(decode(output).unwrap(), (base, input.to_vec()));
    }
}

#[test]
fn preserves_leading_zeroes() {
    let input = b"\x00\x00\x00yes mani !";
    let test_cases = vec![
        (Identity, "\x00\x00\x00\x00yes mani !"),
        (Base2, "000000000000000000000000001111001011001010111001100100000011011010110000101101110011010010010000000100001"),
        (Base8, "700000000362625631006654133464440102"),
        (Base10, "9000573277761329450583662625"),
        (Base16Lower, "f000000796573206d616e692021"),
        (Base16Upper, "F000000796573206D616E692021"),
        (Base32Lower, "baaaaa6lfomqg2yloneqcc"),
        (Base32Upper, "BAAAAA6LFOMQG2YLONEQCC"),
        (Base32PadLower, "caaaaa6lfomqg2yloneqcc==="),
        (Base32PadUpper, "CAAAAA6LFOMQG2YLONEQCC==="),
        (Base32HexLower, "v00000ub5ecg6qobed4g22"),
        (Base32HexUpper, "V00000UB5ECG6QOBED4G22"),
        (Base32HexPadLower, "t00000ub5ecg6qobed4g22==="),
        (Base32HexPadUpper, "T00000UB5ECG6QOBED4G22==="),
        (Base32Z, "hyyyyy6mfqcog4amqpronn"),
        (Base58Flickr, "Z1117Pznk19XTTzBtx"),
        (Base58Btc, "z1117paNL19xttacUY"),
        (Base64, "mAAAAeWVzIG1hbmkgIQ"),
        (Base64Pad, "MAAAAeWVzIG1hbmkgIQ=="),
        (Base64Url, "uAAAAeWVzIG1hbmkgIQ"),
        (Base64UrlPad, "UAAAAeWVzIG1hbmkgIQ=="),
    ];

    for (base, output) in test_cases {
        assert_eq!(encode(base, input), output);
        assert_eq!(decode(output).unwrap(), (base, input.to_vec()));
    }
}
