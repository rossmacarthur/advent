#![allow(unstable_name_collisions)]

use std::collections::hash_map::Entry;

use advent::prelude::*;

fn parse_input(input: &str) -> Map<2> {
    let iter = (0..).map_while(|c| {
        let mut lines = input.lines().map(|line| line.chars());
        let col = iter::once(lines.next()?.nth(c)?)
            .chain(lines.filter_map(move |mut chars| chars.nth(c)));
        Some(col)
    });

    let hallway = [None; 11];
    let rooms: [[_; 2]; 4] = iter
        .flatten()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            _ => None,
        })
        .map(Some)
        .array_chunked()
        .next_array()
        .unwrap();

    Map { hallway, rooms }
}

fn default_input() -> Map<2> {
    parse_input(include_str!("input/23.txt"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Map<const R: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; R]; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn room(&self) -> usize {
        match *self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    fn energy(&self) -> usize {
        match *self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

const fn target_rooms<const R: usize>() -> [[Option<Amphipod>; R]; 4] {
    [
        [Some(Amphipod::Amber); R],
        [Some(Amphipod::Bronze); R],
        [Some(Amphipod::Copper); R],
        [Some(Amphipod::Desert); R],
    ]
}

// Returns an iterator over all the combinations of moving amphipods *out*.
fn move_out<const R: usize>(map: &Map<R>) -> impl Iterator<Item = (Map<R>, usize)> + '_ {
    map.rooms
        .iter()
        .enumerate()
        .filter(|(r, room)| {
            // Filter out amphipods that are correctly arranged.
            room.iter().filter_map(|&p| p).any(|a| a.room() != *r)
        })
        .filter_map(|(r, room)| {
            // Find the first amphipod in the room (only the first can move).
            room.iter()
                .enumerate()
                .find_map(|(b, p)| p.map(|a| (a, r, b)))
        })
        .flat_map(move |(a, r, b)| {
            // Now we have the amphipod's room index and the bed position
            // within that room.

            // Convert room index to a hallway position.
            let h0 = 2 * r + 2;

            // Now find all the possible places to move to.
            let hmax = map.hallway.len();
            let left = (0..=h0).rev().take_while(|&h| map.hallway[h].is_none());
            let right = (h0..hmax).take_while(|&h| map.hallway[h].is_none());

            // Loop through all of these places.
            left.chain(right)
                // Filter out bad hallway positions.
                .filter(|&h| ![2, 4, 6, 8].contains(&h))
                // Calculate the cost of moving an amphipod and return the new
                // state and the cost.
                .map(move |h1| {
                    let mut next = *map;
                    let steps = max(h0, h1) - min(h0, h1) + (b + 1);
                    let cost = a.energy() * steps;
                    mem::swap(&mut next.rooms[r][b], &mut next.hallway[h1]);
                    (next, cost)
                })
        })
}

// Returns an iterator over all the combinations of moving amphipods *in*.
fn move_in<const R: usize>(map: &Map<R>) -> impl Iterator<Item = (Map<R>, usize)> + '_ {
    map.hallway
        .iter()
        .enumerate()
        .filter_map(|(h, p)| p.map(|a| (a, h, a.room())))
        .filter_map(|(a, h0, r)| {
            // Now we have the hallway position of the amphipod and the index of
            // the room it needs to go into.

            // Check if there are any incorrect amphipods in the room.
            if map.rooms[r]
                .iter()
                .filter_map(|&p| p)
                .any(|a| a.room() != r)
            {
                return None;
            }

            // Convert room index to a hallway position.
            let h1 = 2 * r + 2;

            // Check if the path to the room is clear.
            let (steps, path) = match h0.cmp(&h1) {
                Ordering::Equal => unreachable!(),
                Ordering::Less => (h1 - h0, &map.hallway[h0 + 1..=h1]),
                Ordering::Greater => (h0 - h1, &map.hallway[h1..h0]),
            };
            if path.iter().any(|p| p.is_some()) {
                return None;
            }

            // Find the furthest bed in the room, there has to be space!
            let b = map.rooms[r].iter().rposition(|p| p.is_none()).unwrap();

            // Calculate the cost of moving an amphipod and return the new
            // state and the cost.
            let mut next = *map;
            let steps = steps + b + 1;
            let cost = a.energy() * steps;
            mem::swap(&mut next.hallway[h0], &mut next.rooms[r][b]);
            Some((next, cost))
        })
}

fn solve<const R: usize>(map: Map<R>) -> usize {
    let mut pq = BinaryHeap::new();
    let mut visited = HashMap::new();
    pq.push((Reverse(0), map));
    while let Some((Reverse(cost), map)) = pq.pop() {
        if map.rooms == target_rooms() {
            return cost;
        }
        for (m, c) in move_out(&map).chain(move_in(&map)) {
            let cost = cost + c;
            match visited.entry(m) {
                Entry::Occupied(mut entry) => {
                    // If this map already exists then we only keep it if the
                    // cost is less.
                    if cost < *entry.get() {
                        entry.insert(cost);
                        pq.push((Reverse(cost), m));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(cost);
                    pq.push((Reverse(cost), m));
                }
            }
        }
    }
    panic!("no arrangement found")
}

fn part1(map: Map<2>) -> usize {
    solve(map)
}

fn part2(map: Map<2>) -> usize {
    let Map { hallway, rooms } = map;
    // Inserts the following
    //    #D#C#B#A#
    //    #D#B#A#C#
    let fill = [
        [Amphipod::Desert, Amphipod::Desert],
        [Amphipod::Copper, Amphipod::Bronze],
        [Amphipod::Bronze, Amphipod::Amber],
        [Amphipod::Amber, Amphipod::Copper],
    ];
    let rooms = rooms
        .into_iter()
        .enumerate()
        .map(|(i, room)| [room[0], Some(fill[i][0]), Some(fill[i][1]), room[1]])
        .next_array()
        .unwrap();
    solve(Map { hallway, rooms })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    );
    assert_eq!(part1(input), 12521);
    assert_eq!(part2(input), 44169);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 16157);
    assert_eq!(part2(input), 43481);
}
