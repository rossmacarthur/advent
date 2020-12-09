use criterion::{criterion_group, Criterion};

use advent::day09::*;

fn bench_part1(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day09_part1", |b| b.iter(|| part1(&input)));
}

criterion_group!(benches, bench_part1);
