use criterion::{criterion_group, Criterion};

use advent_2020::day23::*;

fn bench_play(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day23_play", |b| {
        b.iter(|| play(input.to_vec(), 100_000, Some(100_000)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day23_part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_play, bench_part2);
