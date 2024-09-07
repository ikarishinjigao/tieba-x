use criterion::{criterion_group, criterion_main, Criterion};
use crypto::{generate_c3_aid, generate_cuid};

fn bench_generate_c3_aid(c: &mut Criterion) {
    c.bench_function("generate_c3_aid", |b| {
        b.iter(|| generate_c3_aid("52ee55117d525049", "8cf4b37a-b859-4147-aa65-4f4bfd4b7c62"))
    });
}

fn bench_generate_cuid(c: &mut Criterion) {
    c.bench_function("generate_cuid", |b| {
        b.iter(|| generate_cuid("52ee55117d525049"))
    });
}

criterion_group!(benches, bench_generate_c3_aid, bench_generate_cuid);
criterion_main!(benches);
