# advent

[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/advent/build/master)](https://github.com/rossmacarthur/advent/actions?query=workflow%3Abuild)

My [Advent of Code](https://adventofcode.com) solutions. All in Rust of course!

## Getting started

To run a specfic solution just use `cargo run` and pass in the year and the day.
For example, the following will run the solution for 2020 Day 7.

```
cargo run -- --year 2020 --day 7
```

Tests can be run by specifying the package name and the day.

```
cargo test --lib --package advent-2020 day07
```

Benchmarks can be run by specifying the package name and the day.

```
cargo bench --package advent-2020 day07
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
