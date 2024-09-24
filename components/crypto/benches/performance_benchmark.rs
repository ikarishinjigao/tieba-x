use criterion::{criterion_group, criterion_main, Criterion};
use crypto::IdManager;

fn bench_generate_uuid(c: &mut Criterion) {
    c.bench_function("generate_uuid", |b| {
        b.iter(|| IdManager::new().generate_uuid())
    });
}

fn bench_generate_android_id(c: &mut Criterion) {
    c.bench_function("generate_android_id", |b| {
        b.iter(|| IdManager::new().generate_android_id(28))
    });
}

fn bench_generate_c3_aid(c: &mut Criterion) {
    c.bench_function("generate_c3_aid", |b| {
        b.iter(|| {
            IdManager::new()
                .generate_c3_aid("3bf49d47909c5f4b", "8cf4b37a-b859-4147-aa65-4f4bfd4b7c62")
        })
    });
}

fn bench_generate_cuid(c: &mut Criterion) {
    c.bench_function("generate_cuid", |b| {
        b.iter(|| IdManager::new().generate_cuid("3bf49d47909c5f4b"))
    });
}

criterion_group!(
    benches,
    bench_generate_uuid,
    bench_generate_android_id,
    bench_generate_c3_aid,
    bench_generate_cuid
);
criterion_main!(benches);
