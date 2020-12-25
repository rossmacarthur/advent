# advent

My [Advent of Code](https://adventofcode.com) solutions. All in Rust of course!

## Getting started

To run a specfic solution just use `cargo run` and pass in the year and the day.
For example, the following will run the solution for 2020 Day 10.

```
cargo run -- --year 2020 --day 1
```

Tests can be run by specifying the package name and the day.

```
cargo test --package advent-2020 day01
```

Benchmarks can be run by specifying the package name and the day.

```
cargo bench --package advent-2020 day01
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
