mod intcode;

use std::collections::BTreeMap;
use std::iter;

use crate::intcode::{parse_program, Computer, State};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/23.txt"))
}

#[derive(Debug, Clone)]
struct Packet {
    x: i64,
    y: i64,
}

struct Network {
    nics: BTreeMap<i64, Computer>,
    nat: Option<Packet>,
}

impl Computer {
    fn feed_packets(&mut self, ps: impl IntoIterator<Item = Packet>) {
        self.feed(ps.into_iter().flat_map(|Packet { x, y }| [x, y]));
    }

    fn poll(&mut self) -> Option<i64> {
        match match self.try_next_state() {
            Ok(state) => state,
            Err(err) => panic!("oops `{}`", err),
        } {
            State::Yielded(v) => Some(v),
            State::Waiting => None,
            state => panic!("unexpected state `{:?}`", state),
        }
    }

    fn poll_packet(&mut self) -> Option<(i64, Packet)> {
        let addr = self.poll()?;
        let x = self.poll().unwrap();
        let y = self.poll().unwrap();
        Some((addr, Packet { x, y }))
    }

    fn poll_packets(&mut self) -> impl Iterator<Item = (i64, Packet)> + '_ {
        iter::from_fn(|| self.poll_packet())
    }
}

impl Network {
    /// Initialize a new network with 50 NICs running the provided intcode.
    fn new(input: &[i64]) -> Self {
        let nics = (0..50)
            .map(|addr| {
                let mut c = Computer::new(input.to_vec());
                c.feed([addr, -1]);
                (addr, c)
            })
            .collect();
        Self { nics, nat: None }
    }

    /// Poll each NIC until it is `Waiting` and return all the packets.
    fn poll(&mut self) -> BTreeMap<i64, Vec<Packet>> {
        let mut packets = BTreeMap::new();
        for (addr, p) in self.nics.values_mut().flat_map(Computer::poll_packets) {
            if addr == 255 {
                self.nat = Some(p);
            } else {
                packets.entry(addr).or_insert_with(Vec::new).push(p);
            }
        }
        packets
    }

    /// Deliver the provided packets to NICs.
    fn deliver(&mut self, mut packets: BTreeMap<i64, Vec<Packet>>) {
        for (addr, nic) in self.nics.iter_mut() {
            if let Some(ps) = packets.remove(addr) {
                nic.feed_packets(ps);
            }
        }
        debug_assert!(packets.is_empty());
    }

    fn deliver_nat_packet(&mut self) -> Packet {
        let p = self.nat.clone().unwrap();
        self.nics.get_mut(&0).unwrap().feed_packets([p.clone()]);
        p
    }
}

fn part1(input: &[i64]) -> i64 {
    let mut net = Network::new(input);
    loop {
        let packets = net.poll();
        if let Some(p) = net.nat {
            break p.y;
        }
        net.deliver(packets);
    }
}

fn part2(input: &[i64]) -> i64 {
    let mut net = Network::new(input);
    let mut prev: Option<Packet> = None;
    loop {
        let packets = net.poll();
        // We just polled all the NICs until they were all `Waiting` so if there
        // are no packets then the network must be idle.
        if packets.is_empty() {
            // Check if we previously sent a packet with this `y` value.
            // Otherwise deliver the NAT packet.
            match (&net.nat, &prev) {
                (Some(p), Some(prev)) if p.y == prev.y => break p.y,
                _ => prev = Some(net.deliver_nat_packet()),
            }
        }
        net.deliver(packets);
    }
}

fn main() {
    let input = default_input();
    let mut run = advent::start();
    run.part(|| part1(&input));
    run.part(|| part2(&input));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 18982);
    assert_eq!(part2(&input), 11088);
}
