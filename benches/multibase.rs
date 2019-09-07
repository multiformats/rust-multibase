use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multibase::*;
use rand::Rng;

fn bench_encode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..1024).into_iter().map(|_| rng.gen()).collect();

    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = encode(Base::Base58btc, &data);
            black_box(result);
        })
    });

    c.bench_function("base32", |b| {
        b.iter(|| {
            let result = encode(Base::Base32UpperNoPad, &data);
            black_box(result);
        })
    });

    c.bench_function("base64", |b| {
        b.iter(|| {
            let result = encode(Base::Base64UpperNoPad, &data);
            black_box(result);
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let data: Vec<usize> = (0..1024).into_iter().map(|_| rng.gen()).collect();
    let base32 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let base58 = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let base64 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut base32_data: String = data.iter().map(|i| base32[i % 31] as char).collect();
    base32_data.insert(0, Base::Base32UpperNoPad.code());
    let mut base58_data: String = data.iter().map(|i| base58[i % 57] as char).collect();
    base58_data.insert(0, Base::Base58btc.code());
    let mut base64_data: String = data.iter().map(|i| base64[i % 64] as char).collect();
    base64_data.insert(0, Base::Base64UpperNoPad.code());

    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = decode(&base58_data).unwrap();
            black_box(result);
        })
    });

    c.bench_function("base32", |b| {
        b.iter(|| {
            let result = decode(&base32_data).unwrap();
            black_box(result);
        })
    });

    c.bench_function("base64", |b| {
        b.iter(|| {
            let result = decode(&base64_data).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
