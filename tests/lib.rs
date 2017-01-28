extern crate multibase;

use multibase::*;

#[test]
fn test_bases_code() {
    assert_eq!(Base::Base1.code(), "1");
    assert_eq!(Base::Base64urlpad.code(), "U");
}

#[test]
fn test_round_trip() {
    let strings = vec![
        "helloworld",
        "we all want decentralization",
        "zdj7WfBb6j58iSJuAzDcSZgy2SxFhdpJ4H87uvMpfyN6hRGyH",
    ];

    for s in strings {
        assert_eq!(
            decode(
                encode(Base::Base58btc, s).unwrap()
            ).unwrap(),
            (Base::Base58btc, s.as_bytes().to_vec())
        );
    }

    let val: Vec<u8> = vec![1, 2, 3, 98, 255, 255, 255];
    assert_eq!(
        decode(
            encode(Base::Base64url, val.clone()).unwrap()
        ).unwrap(),
        (Base::Base64url, val)
    )
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code("1").unwrap(), Base::Base1);
    assert_eq!(Base::from_code("U").unwrap(), Base::Base64urlpad);
}

#[test]
fn test_encode() {
    let id = "Decentralize everything!!";

    assert_eq!(encode(Base::Base16, id).unwrap(),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base::Base16, id.to_string()).unwrap(),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base::Base16, id.as_bytes()).unwrap(),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base::Base58btc, id).unwrap(),
               "zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe");

    let id2 = "yes mani !";

    assert_eq!(encode(Base::Base2, id2).unwrap(),
               "01111001011001010111001100100000011011010110000101101110011010010010000000100\
                001");
    assert_eq!(encode(Base::Base8, id2).unwrap(),
               "7171312714403326055632220041");
    assert_eq!(encode(Base::Base10, id2).unwrap(),
               "9573277761329450583662625");
    assert_eq!(encode(Base::Base16, id2).unwrap(), "f796573206d616e692021");
    assert_eq!(encode(Base::Base32hex, id2).unwrap(), "vf5in683dc5n6i811");
    assert_eq!(encode(Base::Base32, id2).unwrap(), "bpfsxgidnmfxgsibb");
    assert_eq!(encode(Base::Base32z, id2).unwrap(), "hxf1zgedpcfzg1ebb");
    assert_eq!(encode(Base::Base58flickr, id2).unwrap(), "Z7Pznk19XTTzBtx");
    assert_eq!(encode(Base::Base58btc, id2).unwrap(), "z7paNL19xttacUY");
}

#[test]
fn test_decode() {
    let id = b"Decentralize everything!!";

    assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121").unwrap(),
               (Base::Base16, id.to_vec()));

    assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121".to_string()).unwrap(),
               (Base::Base16, id.to_vec()));


    assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121".as_bytes()).unwrap(),
               (Base::Base16, id.to_vec()));

    assert_eq!(decode("zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe").unwrap(),
               (Base::Base58btc, id.to_vec()));

    let id2 = b"yes mani !";

    assert_eq!(decode("011110010110010101110011001000000110110101100001011011100110100100100\
                       00000100001")
               .unwrap(),
               (Base::Base2, id2.to_vec()));
    assert_eq!(decode("7171312714403326055632220041").unwrap(),
               (Base::Base8, id2.to_vec()));
    assert_eq!(decode("9573277761329450583662625").unwrap(),
               (Base::Base10, id2.to_vec()));
    assert_eq!(decode("f796573206d616e692021").unwrap(),
               (Base::Base16, id2.to_vec()));
    assert_eq!(decode("vf5in683dc5n6i811").unwrap(),
               (Base::Base32hex, id2.to_vec()));
    assert_eq!(decode("bpfsxgidnmfxgsibb").unwrap(),
               (Base::Base32, id2.to_vec()));
    assert_eq!(decode("hxf1zgedpcfzg1ebb").unwrap(),
               (Base::Base32z, id2.to_vec()));
    assert_eq!(decode("Z7Pznk19XTTzBtx").unwrap(),
               (Base::Base58flickr, id2.to_vec()));
    assert_eq!(decode("z7paNL19xttacUY").unwrap(),
               (Base::Base58btc, id2.to_vec()));

    // Fails
    assert_eq!(decode("Lllll"), Err(Error::UnkownBase));
    assert_eq!(decode("Ullll"), Err(Error::UnsupportedBase));

    assert_eq!(decode("z7pa_L19xttacUY"), Err(Error::InvalidBaseString))
}
