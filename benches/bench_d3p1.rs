use aoc_2024_rust::d3p1 as day;
use criterion::{criterion_group, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_d3p1(c: &mut Criterion) {
    let input = day::read_input();
    let mut group = c.benchmark_group("d3p1");
    group.bench_function(BenchmarkId::new("iter", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_iter(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("regex", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_regex(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_d3p1);
