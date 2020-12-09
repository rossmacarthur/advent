mod day01;
mod day02;
mod day05;
mod day09;

criterion::criterion_main!(
    crate::day01::benches,
    crate::day02::benches,
    crate::day05::benches,
    crate::day09::benches
);
