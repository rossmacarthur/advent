use criterion::{criterion_group, Criterion};

use advent::day01::*;

fn bench_part1(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
