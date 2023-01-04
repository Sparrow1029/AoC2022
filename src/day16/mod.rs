use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};

const PART1_TIME_LIMIT: u32 = 30;
const PART2_TIME_LIMIT: u32 = 26;

#[derive(Debug)]
pub struct Valve<'a> {
    pub flow: u32,
    pub adjacent: HashSet<&'a str>,
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Get lowest cost to move from a valve to another valve
fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    // shortest path:
    // Dijkstra's algorithm
    // nodes in the priority queue are sorted so the lowest cost gets popped first
    let mut pq = BinaryHeap::new();
    // prevent backtracking by keeping track of seen valve rooms
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: 0,
        curr: from,
    });
    seen.insert(from);

    while let Some(Node { cost, curr }) = pq.pop() {
        if curr == to {
            return cost;
        }
        for next in map[curr].adjacent.iter() {
            // HashSet returns `true` if item was not already in the set
            if seen.insert(next) {
                pq.push(Node {
                    cost: cost + 1,
                    curr: next,
                });
            }
        }
    }
    u32::MAX
}

/// map shortest distance from "AA" to any flowing valve
/// map shortest distance from any flowing valve to any other
fn min_distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        // only keep flowing valves
        .filter(|(_, valve)| valve.flow > 0)
        // get names of those valves
        .map(|(&name, _)| name)
        // iter over every combo of 2 flowing valves
        .tuple_combinations()
        // record shorted distance between those 2
        // (and the dist from "AA" to flowing valve bc that is the starting point)
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            // from AA to name1
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            // from AA to name2
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));
            let dist = min_cost(name1, name2, map);
            // from name1 to name2
            acc.insert((name1, name2), dist);
            // from name2 to name1
            acc.insert((name2, name1), dist);

            acc
        })
}

/// Contain current state of open valves for simulation
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: u32,
    relieved: u32,
}

/// All flowing valves are open, wait until time limit is reached
fn wait_until_times_up(
    max_time: u32,
    elapsed: u32,
    relieved: u32,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve>,
) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_minute: u32 = opened.iter().map(|name| &map[name].flow).sum();
    relieved + (relieved_per_minute * time_left)
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input).finish().unwrap().1;
    let dist_map = min_distances(&map); // key: (from, to), value: move_cost
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    let mut max_relieved = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });
    // current position doesn't matter for `seen`
    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(State {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        // If all flowing valves are already open, wait until the end
        if opened.len() == flowing.len() || elapsed >= PART1_TIME_LIMIT {
            let relieved_at_end =
                wait_until_times_up(PART1_TIME_LIMIT, elapsed, relieved, &opened, &map);
            max_relieved = max_relieved.max(relieved_at_end);
            continue;
        }
        // for every unopened valve, run the simulation/traverse graph
        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            // how long would moving to dest take? +1 to open the valve
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            // if openeing the dest valve would exceed the time limit, wait until the end
            if new_elapsed >= PART1_TIME_LIMIT {
                let relieved_at_end =
                    wait_until_times_up(PART1_TIME_LIMIT, elapsed, relieved, &opened, &map);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            // relieve pressure of opened valves while we move to dest and open the valve there
            let relieved_per_minute: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + relieved_per_minute * cost;
            // add opened valve to opened valve HashSet
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                });
            }
        }
    }
    max_relieved
}

fn part2(input: &str) -> u32 {
    let map = parse_input(input).finish().unwrap().1;
    let dist_map = min_distances(&map); // key: (from, to), value: move_cost
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    // key: opened, val: relieved_at_end
    let mut max_relieved_states: HashMap<BTreeSet<&str>, u32> = HashMap::new();

    let mut q = VecDeque::new();
    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    while let Some(State {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        let relieved_at_end =
            wait_until_times_up(PART2_TIME_LIMIT, elapsed, relieved, &opened, &map);
        // record state. only update state if it beats the `relieved_at_end` value
        max_relieved_states
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        // if all flowing valves are opened or time limit is reached, skip
        if opened.len() == flowing.len() || elapsed >= PART2_TIME_LIMIT {
            continue;
        }
        // for every unopened valve, run simulation
        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            // how long would moving to dest take? +1 to open the valve
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            // if openeing the dest valve would exceed the time limit, wait until the end
            if new_elapsed >= PART2_TIME_LIMIT {
                continue;
            }

            // relieve pressure of opened valves while we move to dest and open the valve there
            let relieved_per_minute: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + relieved_per_minute * cost;

            // add opened valve to opened valve HashSet
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            q.push_back(State {
                opened: new_opened,
                curr: dest,
                elapsed: new_elapsed,
                relieved: new_relieved,
            });
        }
    }

    max_relieved_states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Valve>> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (_, (_, id, _, flow, _, _, adjacent)) = tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            cc::u32,
            alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
            alt((tag("valves "), tag("valve "))),
            separated_list0(tag(", "), alpha1),
        ))(line)?;
        valves.insert(
            id,
            Valve {
                flow,
                adjacent: adjacent.into_iter().collect(),
            },
        );
    }
    Ok((input, valves))
}

pub fn run() {
    println!("\n=== Day 16 ===");
    let input = include_str!("input.txt");
    // for (k, v) in parse_input(input).finish().unwrap().1.iter() {
    //     println!("{k}: {v:?}");
    // }
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
