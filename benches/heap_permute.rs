
use criterion::{criterion_group, criterion_main, Criterion};
use heap_permute::{HeapPermutor, PermuteIter};

const STRING: &'static str = "ABCDEF";

pub fn criterion_benchmark(c: &mut Criterion) {
    // Raw permute
    let mut string = STRING.bytes().collect::<Vec<u8>>();
    let mut permutor = HeapPermutor::new(string.len());

    c.bench_function("raw_permute", |b| b.iter(|| unsafe {permutor.permute(&mut string)}));

    // Iter permute
    let mut iter = PermuteIter::from(STRING);

    c.bench_function("iter_permute", |b| b.iter(|| iter.next()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);