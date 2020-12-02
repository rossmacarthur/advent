use criterion::{criterion_group, criterion_main, Criterion};

use advent::day02::*;

fn bench_valid_with_count_policy(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day02_valid_with_count_policy", |b| {
        b.iter(|| valid_with_count_policy(&input))
    });
}

fn bench_valid_with_position_policy(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day02_valid_with_position_policy", |b| {
        b.iter(|| valid_with_position_policy(&input))
    });
}

criterion_group!(
    benches,
    bench_valid_with_count_policy,
    bench_valid_with_position_policy
);
criterion_main!(benches);
