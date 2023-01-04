use serde_json::{from_str, Value};
use std::cmp::Ordering;

use itertools::Itertools;

const DEBUG: bool = false;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<[Packet; 2]> {
    input
        .split("\n\n")
        .map(|s| {
            let split = s.trim().split('\n').collect_vec();
            [
                parse_packet_from_value(from_str::<Value>(split[0]).expect("Bad JSON")),
                parse_packet_from_value(from_str::<Value>(split[1]).expect("Bad JSON")),
            ]
        })
        .collect_vec()
}

fn parse_packet_from_value(v: Value) -> Packet {
    match v {
        Value::Array(arr) => Packet::List(
            arr.iter()
                .map(|v| parse_packet_from_value(v.clone()))
                .collect_vec(),
        ),
        Value::Number(num) => Packet::Num(num.as_f64().unwrap() as u8),
        _ => unreachable!("Should be no other types..."),
    }
}

fn part_1(pairs: &[[Packet; 2]]) -> usize {
    pairs
        .iter()
        .positions(|[left, right]| left < right)
        .map(|idx| idx + 1) // indices are 1-based
        .sum()
}

fn part_2(pairs: &[[Packet; 2]]) -> usize {
    let mut packets: Vec<_> = pairs.iter().flatten().collect();
    let div_1 = parse_packet_from_value(from_str::<Value>("[[2]]").unwrap());
    let div_2 = parse_packet_from_value(from_str::<Value>("[[6]]").unwrap());

    packets.push(&div_1);
    packets.push(&div_2);

    packets.sort_unstable();
    packets
        .into_iter()
        .positions(|packet| packet == &div_1 || packet == &div_2)
        .map(|idx| idx + 1)
        .product()
}

pub fn run() {
    println!("\n=== Day 13 ===");
    let input = include_str!("input.txt");
    let pairs = parse(input);
    if DEBUG {
        for p in &pairs {
            println!("{:?}\n{:?}\n", p[0], p[1]);
        }
    }
    println!("Part 1: {}", part_1(&pairs));
    println!("Part 2: {}", part_2(&pairs));
}
