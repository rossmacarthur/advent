# advent

[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/advent/build/master)](https://github.com/rossmacarthur/advent/actions?query=workflow%3Abuild)

My 🎅 [Advent of Code](https://adventofcode.com) solutions. Includes a runner
and benchmarker with free Christmas trees 🎄.

<img width="429" alt="screenshot of cargo run with bench" src="https://user-images.githubusercontent.com/17109887/145716328-3d7caf20-0a15-4c2e-89f1-ffed6ec22fe0.png">

## Getting started

To run a specific solution just use the provided `cargo advent` alias and pass
in the year and day. For example, the following will run the solution for 2020
day 18.

```
cargo advent -y 2020 -d 18 run
```

Tests can be run using the `test` subcommand.

```
cargo advent -y 2020 -d 18 test
```

Benchmarks can be run by passing `--bench` to the binary.

```
cargo advent -y 2020 -d 18 run -- -- --bench
```

Note the double `--`. Arguments after the first `--` will be passed to Cargo and
arguments after the second `--` will be passed to the actual binary. For
example if we wanted the JSON output of the solution.

```
cargo advent -y 2020 -d 18 run -- --features=json -- --output json
```

All of the above will be built using `--release`.

## Development

Use the following to add a [template](./crates/cli/src/template.rs) for a new
solution. To fetch the input the `ADVENT_SESSION` environment variable needs to
be set which can be extracted from a logged in Advent of Code browser session
under the cookie name "session".

```
cargo advent -y 2022 -d 1 new
```

Open the browser for the given problem

```
cargo advent -y 2020 -d 7 new
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
