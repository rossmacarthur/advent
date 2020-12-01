use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day01::*;

fn bench_sum_two_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_sum_two_with_default", |b| {
        b.iter(|| solve_sum_two(&input))
    });
}

fn bench_sum_three_default(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day01_sum_three_with_default", |b| {
        b.iter(|| solve_sum_three(&input))
    });
}

fn bench_sum_two_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_sum_two_with_random", |b| {
        b.iter(|| solve_sum_two(&input))
    });
}

fn bench_sum_three_random(c: &mut Criterion) {
    let input = random_input(1000);
    c.bench_function("day01_sum_three_with_random", |b| {
        b.iter(|| solve_sum_three(&input))
    });
}

criterion_group!(
    benches,
    bench_sum_two_default,
    bench_sum_three_default,
    bench_sum_two_random,
    bench_sum_three_random
);
criterion_main!(benches);
