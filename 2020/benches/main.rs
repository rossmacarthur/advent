mod day01;
mod day02;
mod day05;
mod day09;
mod day11;
mod day13;
mod day15;
mod day23;

criterion::criterion_main!(
    day01::benches,
    day02::benches,
    day05::benches,
    day09::benches,
    day11::benches,
    day13::benches,
    day15::benches,
    day23::benches,
);
