use aoc_2024_rust::d7p2 as day;
use criterion::{criterion_group, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let input = day::read_input();
    let mut group = c.benchmark_group("d7p2");

    group.bench_function(BenchmarkId::new("op_generation", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_generator(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("op_generation_optimized", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_generator_optimized(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("op_recursion", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_recursion(data)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(
        BenchmarkId::new("op_recursion_numeric_concat", "input"),
        |b| {
            b.iter_batched(
                || input.clone(),
                |data| black_box(day::solve_recursion_numeric_concatenation(data)),
                BatchSize::SmallInput,
            )
        },
    );

    group.bench_function(BenchmarkId::new("op_recursion_parallel", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| black_box(day::solve_recursion_parallel(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench);
