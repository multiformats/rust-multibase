use multibase::Base::*;
use multibase::{decode, encode, Base, Error};

#[test]
fn test_bases_code() {
    assert_eq!(Base2.code(), '0');
}

#[test]
fn test_round_trip() {
    let slices: &[&[u8]] = &[
        b"helloworld",
        b"we all want decentralization",
        b"zdj7WfBb6j58iSJuAzDcSZgy2SxFhdpJ4H87uvMpfyN6hRGyH",
    ];

    for s in slices {
        assert_eq!(
            decode(encode(Base58btc, s)).unwrap(),
            (Base58btc, s.to_vec())
        );
    }

    let val = vec![1, 2, 3, 98, 255, 255, 255];
    assert_eq!(decode(encode(Base64UrlUpperNoPad, &val)).unwrap(), (Base64UrlUpperNoPad, val))
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code('0').unwrap(), Base2);
}

#[test]
fn test_encode() {
    let id = b"Decentralize everything!!";

    assert_eq!(
        encode(Base16Lower, id),
        "f446563656e7472616c697a652065766572797468696e672121"
    );

    assert_eq!(
        encode(Base16Lower, String::from_utf8(id.to_vec()).unwrap()),
        "f446563656e7472616c697a652065766572797468696e672121"
    );

    assert_eq!(
        encode(Base16Lower, id.to_vec()),
        "f446563656e7472616c697a652065766572797468696e672121"
    );

    assert_eq!(encode(Base58btc, id), "zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe");

    let id2 = b"yes mani !";

    assert_eq!(
        encode(Base2, id2),
        "01111001011001010111001100100000011011010110000101101110011010010010000000100\
         001"
    );
    assert_eq!(encode(Base8, id2), "7171312714403326055632220041");
    assert_eq!(encode(Base10, id2), "9573277761329450583662625");
    assert_eq!(encode(Base16Lower, id2), "f796573206d616e692021");
    assert_eq!(encode(Base58flickr, id2), "Z7Pznk19XTTzBtx");
    assert_eq!(encode(Base58btc, id2), "z7paNL19xttacUY");
}

#[test]
fn preserves_leading_zeroes() {
    let id2 = b"\x00\x00\x00yes mani !";

    assert_eq!(encode(Base2, id2), "00001111001011001010111001100100000011011010110000101101110011010010010000000100001");
    assert_eq!(encode(Base8, id2), "7000171312714403326055632220041");
    assert_eq!(encode(Base10, id2), "9000573277761329450583662625");
    assert_eq!(encode(Base16Upper, id2), "F000796573206D616E692021");
    assert_eq!(encode(Base16Lower, id2), "f000796573206d616e692021");
    assert_eq!(encode(Base32UpperNoPad, id2), "BAAAAA6LFOMQG2YLONEQCC");
    assert_eq!(encode(Base32UpperPad, id2),   "CAAAAA6LFOMQG2YLONEQCC===");
    assert_eq!(encode(Base58flickr, id2), "Z1117Pznk19XTTzBtx");
    assert_eq!(encode(Base58btc, id2),    "z1117paNL19xttacUY");
    assert_eq!(encode(Base64UpperNoPad, id2),    "mAAAAeWVzIG1hbmkgIQ");
    assert_eq!(encode(Base64UpperPad, id2),      "MAAAAeWVzIG1hbmkgIQ==");
    assert_eq!(encode(Base64UrlUpperNoPad, id2), "uAAAAeWVzIG1hbmkgIQ");
    assert_eq!(encode(Base64UrlUpperPad, id2),   "UAAAAeWVzIG1hbmkgIQ==");

    let (base, decoded) = decode("z1117paNL19xttacUY").unwrap();
    assert_eq!(base, Base58btc);
    assert_eq!(&decoded, id2)
}


#[test]
fn test_decode() {
    let id = b"Decentralize everything!!";

    assert_eq!(
        decode("f446563656e7472616c697a652065766572797468696e672121").unwrap(),
        (Base16Lower, id.to_vec())
    );

    assert_eq!(
        decode("f446563656e7472616c697a652065766572797468696e672121".to_string()).unwrap(),
        (Base16Lower, id.to_vec())
    );

    assert_eq!(
        decode("zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe").unwrap(),
        (Base58btc, id.to_vec())
    );

    let id2 = b"yes mani !";

    assert_eq!(
        decode(
            "011110010110010101110011001000000110110101100001011011100110100100100\
             00000100001"
        )
        .unwrap(),
        (Base2, id2.to_vec())
    );
    assert_eq!(
        decode("7171312714403326055632220041").unwrap(),
        (Base8, id2.to_vec())
    );
    assert_eq!(
        decode("9573277761329450583662625").unwrap(),
        (Base10, id2.to_vec())
    );
    assert_eq!(
        decode("f796573206d616e692021").unwrap(),
        (Base16Lower, id2.to_vec())
    );
    assert_eq!(
        decode("Z7Pznk19XTTzBtx").unwrap(),
        (Base58flickr, id2.to_vec())
    );
    assert_eq!(
        decode("z7paNL19xttacUY").unwrap(),
        (Base58btc, id2.to_vec())
    );

    assert_eq!(decode("mZg").unwrap(), (Base64UpperNoPad, b"f".to_vec()));
    assert_eq!(decode("MZg==").unwrap(), (Base64UpperPad, b"f".to_vec()));
    assert_eq!(decode("uZg").unwrap(), (Base64UrlUpperNoPad, b"f".to_vec()));
    assert_eq!(decode("UZg==").unwrap(), (Base64UrlUpperPad, b"f".to_vec()));

    assert_eq!(decode("L1111"), Err(Error::UnknownBase));
    assert_eq!(decode("z7pa_L19xttacUY"), Err(Error::InvalidBaseString));
}
