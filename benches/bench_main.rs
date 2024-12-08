use criterion::criterion_main;

mod bench_d1p1;
mod bench_d1p2;
mod bench_d2p1;
mod bench_d2p2;
mod bench_d3p1;
mod bench_d7p1;
mod bench_d7p2;

criterion_main!(
    bench_d1p1::benches,
    bench_d1p2::benches,
    bench_d2p1::benches,
    bench_d2p2::benches,
    bench_d3p1::benches,
    // TODO add other benches
    bench_d7p1::benches,
    bench_d7p2::benches,
);
