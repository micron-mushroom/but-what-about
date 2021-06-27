
use criterion::{criterion_group, criterion_main, Criterion};
use heap_permute::{PermuteIter};

const STRING: &'static str = "ABCDEF";

pub fn criterion_benchmark(c: &mut Criterion) {
    // Iter permute
    let mut iter = PermuteIter::from(STRING);

    c.bench_function("iter_permute", |b| b.iter(|| iter.next()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);