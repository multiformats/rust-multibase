use multibase::Base::*;
use multibase::{decode, encode, Base, Error};

#[test]
fn test_bases_code() {
    assert_eq!(Base2.code(), '0');
    //assert_eq!(Base32hexUpper.code(), 'V');
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
    assert_eq!(decode(encode(Base64url, &val)).unwrap(), (Base64url, val))
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code('0').unwrap(), Base2);
    //assert_eq!(Base::from_code('V').unwrap(), Base32hexUpper);
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

    assert_eq!(decode("mZg").unwrap(), (Base64, b"f".to_vec()));
    assert_eq!(decode("MZg==").unwrap(), (Base64pad, b"f".to_vec()));
    assert_eq!(decode("uZg").unwrap(), (Base64url, b"f".to_vec()));
    assert_eq!(decode("UZg==").unwrap(), (Base64urlpad, b"f".to_vec()));

    assert_eq!(decode("L1111"), Err(Error::UnknownBase));
    assert_eq!(decode("z7pa_L19xttacUY"), Err(Error::InvalidBaseString));
}
