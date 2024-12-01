use aoc_2024_rust::day_1_2 as day;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_d1_p1(c: &mut Criterion) {
    let input = day::read_input();
    let mut group = c.benchmark_group("D1P2");
    group.bench_function(BenchmarkId::new("HashMap", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_d1_p1);
criterion_main!(benches);
