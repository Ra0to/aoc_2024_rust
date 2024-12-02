use aoc_2024_rust::d2p2 as day;
use criterion::{criterion_group, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_d2p2(c: &mut Criterion) {
    let input = day::read_input();
    let mut group = c.benchmark_group("d2p2");
    group.bench_function(BenchmarkId::new("base", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_d2p2);
