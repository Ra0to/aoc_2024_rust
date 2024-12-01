use aoc_2024_rust::day_1_1;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn bench_fibs(c: &mut Criterion) {
    let input = day_1_1::read_input();
    let mut group = c.benchmark_group("Day_1_1");
    group.bench_function(BenchmarkId::new("With_Sort", "input"), |b| {
        b.iter_batched(
            || input.clone(),
            |data| day_1_1::solve(data),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
