use criterion::{criterion_group, criterion_main, Criterion};

use advent::day01::*;

fn bench_sum_two_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part1_with_default", |b| b.iter(|| part1(&input)));
}

fn bench_sum_three_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_part2_with_default", |b| b.iter(|| part2(&input)));
}

fn bench_sum_two_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_part1_with_random", |b| b.iter(|| part1(&input)));
}

fn bench_sum_three_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_part2_with_random", |b| b.iter(|| part2(&input)));
}

criterion_group!(
    benches,
    bench_sum_two_default,
    bench_sum_three_default,
    bench_sum_two_random,
    bench_sum_three_random
);
criterion_main!(benches);
