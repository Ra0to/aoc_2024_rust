use aoc_2024_rust::d1p2 as day;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_d1_p2(c: &mut Criterion) {
    let input = day::read_input();
    let mut group = c.benchmark_group("d1p2");
    group.bench_function(BenchmarkId::new("hash_map", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_hash_map(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("hash_map_optimized", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_hash_map_optimized(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("count_each_time", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_count_each_time(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("count_with_hash", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_count_with_hash(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(
        BenchmarkId::new("hash_map_optimized_without_unzip", "input"),
        |b| {
            b.iter_batched(
                || input.clone(),
                |data| black_box(day::solve_hash_map_optimized_without_unzip(data)),
                BatchSize::SmallInput,
            )
        },
    );
    group.finish();
}

criterion_group!(benches, bench_d1_p2);
criterion_main!(benches);
