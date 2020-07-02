use criterion::{black_box, criterion_group, criterion_main, Criterion};
use try_trie;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tries", |b| b.iter(|| try_trie::compare_tries()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
