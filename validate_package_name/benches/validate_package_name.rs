use criterion::{black_box, criterion_group, criterion_main, Criterion};
use validate_package_name::validate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invalid_scoped_package", |b| {
        b.iter(|| validate(black_box("@vortex/test!")))
    });

    c.bench_function("valid_scoped_package", |b| {
        b.iter(|| validate(black_box("@vortex/test")))
    });

    c.bench_function("valid_normal_package", |b| {
        b.iter(|| validate(black_box("vortex")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);