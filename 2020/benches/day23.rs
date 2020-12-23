use criterion::{criterion_group, Criterion};

use advent::day23::*;

fn bench_play(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day23_play", |b| {
        b.iter(|| play(input.to_vec(), 100_000, Some(100_000)))
    });
}

criterion_group!(benches, bench_play);
