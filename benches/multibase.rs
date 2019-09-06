use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multibase::Base;
use rand::Rng;

fn base32_encode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let alphabet = Base::Base32.alphabet();
    let data: Vec<u8> = (0..1024).into_iter().map(|_| rng.gen()).collect();
    
    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = base_x::encode(alphabet, &data);
            black_box(result);
        })
    });

    c.bench_function("base32", |b| {
        b.iter(|| {
            let alphabet = base32::Alphabet::RFC4648 { padding: false };
            let result = base32::encode(alphabet, &data);
            black_box(result);
        })
    });
}

fn base32_decode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let alphabet = Base::Base32.alphabet();
    let data: String = (0..1024).into_iter().map(|_| {
        let i: usize = rng.gen();
        let ch: char = alphabet[i % alphabet.len()] as char;
        ch
    }).collect();
    
    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = base_x::decode(alphabet, &data).unwrap();
            black_box(result);
        })
    });

    c.bench_function("base32", |b| {
        b.iter(|| {
            let alphabet = base32::Alphabet::RFC4648 { padding: false };
            let result = base32::decode(alphabet, &data).unwrap();
            black_box(result);
        })
    });
}

fn base64_encode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let alphabet = Base::Base64.alphabet();
    let data: Vec<u8> = (0..1024).into_iter().map(|_| rng.gen()).collect();
    
    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = base_x::encode(alphabet, &data);
            black_box(result);
        })
    });

    c.bench_function("base64", |b| {
        b.iter(|| {
            let result = base64::encode(&data);
            black_box(result);
        })
    });
}

fn base64_decode(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let alphabet = Base::Base64.alphabet();
    let data: String = (0..1024).into_iter().map(|_| {
        let i: usize = rng.gen();
        let ch: char = alphabet[i % alphabet.len()] as char;
        ch
    }).collect();
    
    c.bench_function("base_x", |b| {
        b.iter(|| {
            let result = base_x::decode(alphabet, &data).unwrap();
            black_box(result);
        })
    });

    c.bench_function("base64", |b| {
        b.iter(|| {
            let result = base64::decode(&data).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, base32_encode, base32_decode, base64_encode, base64_decode);
criterion_main!(benches);
