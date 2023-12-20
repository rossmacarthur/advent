use std::collections::hash_map::Entry;
use std::ops::ControlFlow;

use advent::prelude::*;

fn parse_input(input: &str) -> Machine<'_> {
    input
        .lines()
        .map(|line| {
            let (input, outputs) = line.split_once(" -> ").unwrap();
            let (name, kind) = match input {
                "broadcaster" => (input, Module::Broadcaster),
                i if i.starts_with('%') => (&i[1..], Module::FlipFlop),
                i if i.starts_with('&') => (&i[1..], Module::Conjunction),
                _ => panic!("invalid input `{}`", input),
            };
            (name, (kind, outputs.split(", ").collect()))
        })
        .collect()
}

fn default_input() -> Machine<'static> {
    parse_input(include_input!(2023 / 20))
}

/// The machine as a map of module names to its type and outputs.
type Machine<'a> = HashMap<&'a str, (Module, Vec<&'a str>)>;

/// The type of module.
#[derive(Debug, Clone, Copy)]
enum Module {
    /// Broadcasts Hi when the button is pressed.
    Broadcaster,
    /// Outputs the current state, the state flips when it receives Lo.
    FlipFlop,
    /// Outputs Lo when all inputs are Hi.
    Conjunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pulse {
    Lo,
    Hi,
}

/// Allows the pulse to flipped using the not operator like `!pulse`.
impl Pulse {
    fn flip(&mut self) -> Self {
        match *self {
            Self::Lo => *self = Self::Hi,
            Self::Hi => *self = Self::Lo,
        }
        *self
    }
}

/// Returns the map of output to inputs.
fn inverse<'a>(machine: &Machine<'a>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut inv = HashMap::new();
    for (&name, (_, outputs)) in machine {
        for &output in outputs {
            inv.entry(output).or_insert_with(Vec::new).push(name);
        }
    }
    inv
}

fn solve<'a, B, F>(machine: Machine<'a>, mut state: B, mut f: F) -> B
where
    F: FnMut(B, &'a str, Pulse, i64) -> ControlFlow<B, B>,
{
    // Stores the queue of modules to be processed and their incoming pulse
    let mut q = VecDeque::new();

    // Stores the states of the flipflops
    //
    //    flipflop -> pulse
    //
    let mut ffs = HashMap::new();

    // Stores the states of the conjunctions
    //
    //     conjunction -> ( input -> pulse )
    //
    let inv = inverse(&machine);
    let mut cjs = machine
        .iter()
        .fold(HashMap::new(), |mut acc, (&name, (module, _))| {
            if let Module::Conjunction = module {
                for &input in &inv[name] {
                    acc.entry(name)
                        .or_insert_with(HashMap::new)
                        .insert(input, Pulse::Lo);
                }
            }
            acc
        });

    // Repeatedly simulate a button press...
    for presses in 0.. {
        // Inject a signal from the button to the broadcaster
        q.push_back(("button", "broadcaster", Pulse::Lo));

        while let Some((prev, name, pulse)) = q.pop_front() {
            // Allow the caller to process every single pulse
            state = match f(state, name, pulse, presses) {
                ControlFlow::Continue(st) => st,
                ControlFlow::Break(st) => return st,
            };
            let Some((module, outputs)) = machine.get(name) else {
                continue;
            };
            let pulse = match module {
                // If the module is a flipflop and the incoming pulse is Lo then
                // its state is flipped, otherwise nothing happens.
                Module::FlipFlop => match pulse {
                    Pulse::Lo => ffs.entry(name).or_insert(Pulse::Lo).flip(),
                    Pulse::Hi => continue,
                },
                // If the module is a conjunction then the incoming pulse is
                // dependent on the state of the other inputs. If they are all
                // Hi then the output is Lo, otherwise it is Hi.
                Module::Conjunction => {
                    let m = cjs.get_mut(name).unwrap();
                    m.insert(prev, pulse);
                    if m.iter().all(|(_, &p)| p == Pulse::Hi) {
                        Pulse::Lo
                    } else {
                        Pulse::Hi
                    }
                }
                // Otherwise if the module is a broadcaster then always
                // broadcast Lo.
                Module::Broadcaster => pulse,
            };

            // Send the pulse to the outputs.
            for &output in outputs {
                q.push_back((name, output, pulse));
            }
        }
    }

    unreachable!()
}

fn part1(machine: Machine<'_>) -> i64 {
    let (lo, hi) = solve(machine, (-1, 0), |(lo, hi), _, pulse, presses| {
        let state = match pulse {
            Pulse::Lo => (lo + 1, hi),
            Pulse::Hi => (lo, hi + 1),
        };
        if presses < 1000 {
            ControlFlow::Continue(state)
        } else {
            ControlFlow::Break(state)
        }
    });
    lo * hi
}

fn part2(machine: Machine<'_>) -> i64 {
    let mut cyclic = HashMap::new();
    let inv = inverse(&machine);
    for &name in &inv["rx"] {
        for &input in &inv[name] {
            cyclic.insert((input, Pulse::Lo), 0);
        }
    }

    solve(machine, 1, move |mut cycle, name, pulse, presses| {
        if let Entry::Occupied(mut e) = cyclic.entry((name, pulse)) {
            if *e.get() == 0 {
                *e.get_mut() = presses;
            } else {
                cycle = lcm(cycle, presses - e.remove());
            }
        }
        if cyclic.is_empty() {
            ControlFlow::Break(cycle)
        } else {
            ControlFlow::Continue(cycle)
        }
    })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
    );
    assert_eq!(part1(input), 32000000);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    );
    assert_eq!(part1(input), 11687500);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 869395600);
    assert_eq!(part2(input), 232605773145467);
}
