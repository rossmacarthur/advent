use criterion::{criterion_group, criterion_main, Criterion};

use advent::day05::*;

fn bench_part2(c: &mut Criterion) {
    let input = default_input();
    c.bench_function("day05_part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part2);
criterion_main!(benches);
