use criterion::{criterion_group, criterion_main, Criterion};
use try_trie;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tries");
    group.bench_function("trie_v1", |b| b.iter(|| try_trie::trie1()));
    group.bench_function("trie_v2", |b| b.iter(|| try_trie::trie2()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
