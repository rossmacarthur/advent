use criterion::{criterion_group, Criterion};

use advent::year2020::day05::*;

fn bench_part2(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("year2020_day05_part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part2);
