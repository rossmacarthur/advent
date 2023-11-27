mod intcode;

use advent::prelude::*;
use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/25.txt"))
}

/// A direction we can take in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// The result of trying to enter a room.
#[derive(Debug, Clone)]
enum EnterResult {
    /// The room details.
    Room(Room),
    /// We failed to enter the room.
    Alert,
    /// Santa gave us the password.
    Password(i64),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Room {
    name: String,
    description: String,
    directions: Vec<Direction>,
    items: Vec<String>,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "north" => Self::North,
            "east" => Self::East,
            "south" => Self::South,
            "west" => Self::West,
            s => panic!("invalid direction `{s}`"),
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::East => "east",
            Self::South => "south",
            Self::West => "west",
        }
    }

    fn rev(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

fn parse_list(lines: &mut Vec<String>, start: &str) -> Option<Vec<String>> {
    lines.iter().position(|line| line == start).map(|i| {
        let j = i + 1;
        let bullets: Vec<_> = lines[j..]
            .iter()
            .take_while(|&line| line.starts_with("- "))
            .map(|line| String::from(&line[2..]))
            .collect();
        lines.drain(i..j + bullets.len());
        bullets
    })
}

impl Computer {
    fn read_lines(&mut self) -> Vec<String> {
        iter::from_fn(|| self.read_line())
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn enter(&mut self, direction: Direction) {
        self.write_line(direction.to_str());
    }

    fn enter_result(&mut self) -> EnterResult {
        let mut lines = self.read_lines();

        // Parse the name
        let name = {
            let heading = lines.remove(0);
            assert!(heading.starts_with("== "));
            String::from(&heading[3..heading.len() - 3])
        };
        // Parse the description
        let description = lines.remove(0);
        // Parse the directions
        let directions: Vec<Direction> = parse_list(&mut lines, "Doors here lead:")
            .unwrap()
            .into_iter()
            .map(|bullet| Direction::from_str(&bullet))
            .collect();
        // Parse the items
        let items = parse_list(&mut lines, "Items here:").unwrap_or_default();

        // Parse the prompt or alert or santa
        match &*lines.remove(0) {
            "Command?" => {
                assert!(lines.is_empty());
                EnterResult::Room(Room {
                    name,
                    description,
                    directions,
                    items,
                })
            }
            line if line.contains("Alert!") => EnterResult::Alert,
            line if line.contains("Analysis complete!") => {
                lines.remove(0);
                let pass = lines
                    .remove(0)
                    .split_whitespace()
                    .find(|s| s.chars().all(|c| c.is_ascii_digit()))
                    .unwrap()
                    .parse()
                    .unwrap();
                assert!(lines.is_empty());
                EnterResult::Password(pass)
            }
            _ => unreachable!(),
        }
    }

    fn take_item(&mut self, item: &str) {
        self.write_line(&format!("take {item}"));
        let mut lines = self.read_lines();
        assert_eq!(lines.remove(0), format!("You take the {item}."));
        assert_eq!(lines.remove(0), "Command?");
        assert!(lines.is_empty());
    }

    fn take_items(&mut self, items: &[String]) {
        for item in items {
            self.take_item(item);
        }
    }

    fn drop_item(&mut self, item: &str) {
        self.write_line(&format!("drop {item}"));
        let mut lines = self.read_lines();
        assert_eq!(lines.remove(0), format!("You drop the {item}."));
        assert_eq!(lines.remove(0), "Command?");
        assert!(lines.is_empty());
    }

    fn drop_items(&mut self, items: &[String]) {
        for item in items {
            self.drop_item(item);
        }
    }
}

const DANGEROUS: &[&str] = &[
    "escape pod",
    "giant electromagnet",
    "infinite loop",
    "molten lava",
    "photons",
];

fn part1(input: Vec<i64>) -> i64 {
    let mut c = Computer::new(input);
    let mut path: Vec<Direction> = Vec::new();
    let mut visited: HashMap<String, HashSet<Direction>> = HashMap::new();
    let mut items = Vec::new();

    loop {
        let room = match c.enter_result() {
            EnterResult::Room(room) => room,
            EnterResult::Alert => break,
            EnterResult::Password(pass) => return pass,
        };

        // Take all the non-dangerous items
        for item in &room.items {
            if !DANGEROUS.contains(&item.as_str()) {
                c.take_item(item);
                items.push(item.clone());
            }
        }

        let visited = visited.entry(room.name).or_insert_with(HashSet::new);
        match room.directions.iter().find(|d| !visited.contains(d)) {
            Some(&d) => {
                c.enter(d);
                path.push(d);
                visited.insert(d);
            }
            None => {
                // There are no more new directions so just backtrack
                let d = path.pop().unwrap();
                c.enter(d.rev());
            }
        }
    }

    // The last direction we were trying to go
    let d = path.last().copied().unwrap();

    // Drop all the items we are holding
    c.drop_items(&items);

    // Simply brute force each combination of items
    for items in items.into_iter().powerset() {
        c.take_items(&items);
        c.enter(d);
        match c.enter_result() {
            EnterResult::Alert => {}
            EnterResult::Password(pass) => return pass,
            EnterResult::Room(_) => unreachable!(),
        };
        c.drop_items(&items);
    }

    panic!("no valid combination of items worked")
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 134349952);
}
