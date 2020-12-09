use criterion::{criterion_group, Criterion};

use advent::day01::*;

fn bench_part1_with_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part1_with_default", |b| b.iter(|| part1(&input)));
}

fn bench_part2_with_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part2_with_default", |b| b.iter(|| part2(&input)));
}

fn bench_part1_with_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_part1_with_random", |b| b.iter(|| part1(&input)));
}

fn bench_part2_with_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_part2_with_random", |b| b.iter(|| part2(&input)));
}

criterion_group!(
    benches,
    bench_part1_with_default,
    bench_part2_with_default,
    bench_part1_with_random,
    bench_part2_with_random
);
