use criterion::{criterion_group, criterion_main, Criterion};
use ctaes::AesContext;

fn aes_benchmark(c: &mut Criterion) {
    let key = [0u8; 16];
    let mut block = [0u8; 16];
    let ctx = AesContext::new(&key);

    c.bench_function("AES Encrypt", |b| b.iter(|| ctx.encrypt(&mut block)));
    c.bench_function("AES Decrypt", |b| b.iter(|| ctx.decrypt(&mut block)));
}

criterion_group!(benches, aes_benchmark);
criterion_main!(benches);
