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

Benchmarks can be run using the  `bench` subcommand.

```
cargo advent -y 2020 -d 18 bench
```

Extra arguments can be passed to both Cargo and the binary. Arguments after the
first `--` argument will be passed to Cargo and arguments after the second `--`
will be passed to the actual binary. For example if we wanted the JSON output of
the benchmark we could run the following.

```
cargo advent -y 2020 -d 18 bench -- --features=json -- --output json
```

All of the above will be built using `--release`.

### New solutions

Use the following to add a [template](./crates/cli/src/template.rs) for a new
solution. To fetch the input the `ADVENT_SESSION` environment variable needs to
be set which can be extracted from a logged in Advent of Code browser session
under the cookie name "session".

```
cargo advent -y 2022 -d 1 new
```

Open the browser for the given problem

```
cargo advent -y 2020 -d 7 open
```

## Using the runner/benchmarker

It is possible to use the runner and benchmarker for your own solutions.

Simply add the crate to Cargo manifest for your solution.
```toml
advent = { git = "https://github.com/rossmacarthur/advent" }
```

Then use the following as your main function.
```rust
fn main() {
    let mut run = advent::with(parse_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}
```

Where `parse_input` is a function that returns any type implementing `Clone`.
And each part function takes that type as a single argument and returns a result
implementing `Display`.

See [template.rs](./crates/cli/src/template.rs) for the template I use  or any
of the solutions in this crate for an example.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
