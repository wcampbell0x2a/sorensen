use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_01(c: &mut Criterion) {
    let data = &[0xa; 1];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b";
    c.bench_function("01 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

pub fn bench_02(c: &mut Criterion) {
    let data = &[0xa; 2];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070";
    c.bench_function("02 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

pub fn bench_03(c: &mut Criterion) {
    let data = &[0xa; 3];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "6a3cf5192354f71615ac51034b3e97c20eda99643fcaf5bbe6d41ad59bd12167";
    c.bench_function("03 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

pub fn bench_04(c: &mut Criterion) {
    let data = &[0xa; 4];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "545c38b0922de19734fbffde62792c37c2aef6a3216cfa472449173165220f7d";
    c.bench_function("04 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

pub fn bench_05(c: &mut Criterion) {
    let data = &[0xa; 5];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "7c370d9536d7d0d6a0f7cd7f9826692acd93e4fb05ba46f7b630b879740343d3";
    c.bench_function("05 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

pub fn bench_06(c: &mut Criterion) {
    let data = &[0xa; 6];
    let total_bits = sorensen::total_bits(black_box(data));
    let sha256_hash = "d088784b7ecb87f1ea17e6f982fa968ffefcc07b79de6ecc548fc00242868da6";
    c.bench_function("06 bytes", |b| {
        b.iter(|| sorensen::decompress(data.len(), total_bits, sha256_hash))
    });
}

criterion_group! {
    name = short;
    config = Criterion::default().sample_size(100);
    targets = bench_01, bench_02, bench_03
}
criterion_group! {
    name = long;
    config = Criterion::default().sample_size(10);
    targets = bench_04, bench_05, bench_06
}
criterion_main!(short, long);
