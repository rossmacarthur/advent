# advent

[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/advent/build/master)](https://github.com/rossmacarthur/advent/actions?query=workflow%3Abuild)

My 🎅 [Advent of Code](https://adventofcode.com) solutions. Includes a runner
and benchmarker with free Christmas trees 🎄.

<img width="429" alt="screenshot of cargo run with bench" src="https://user-images.githubusercontent.com/17109887/145716328-3d7caf20-0a15-4c2e-89f1-ffed6ec22fe0.png">

## Getting started

To run a specific solution just use `cargo run` and pass in the binary name in
`YYYYDD` format. For example, the following will run the solution for 2020 day
18.

```
cargo run --release --bin 202018
```

Benchmarks can be run by passing `--bench` to the binary.

```
cargo run --release --bin 202018 -- --bench
```

Tests can be run using `cargo test`. Passing `--release` is recommended.

```
cargo test --release --bin 202018
```

## Development

Use the following to add a [template](./crates/cli/src/template.rs) for a new
solution, if the `ADVENT_SESSION` environment variable is set then the raw input
will be fetched for the problem.

```
cargo run -p advent-cli -- new -y 2020 -d 7
```

Open the browser for the given problem

```
cargo run -p advent-cli -- open -y 2020 -d 7
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
