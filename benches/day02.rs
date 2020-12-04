use criterion::{criterion_group, criterion_main, Criterion};

use advent::day02::*;

fn bench_valid_with_count_policy(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day02_part1", |b| b.iter(|| part1(&input)));
}

fn bench_valid_with_position_policy(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day02_part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(
    benches,
    bench_valid_with_count_policy,
    bench_valid_with_position_policy
);
criterion_main!(benches);
