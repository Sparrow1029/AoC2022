use std::collections::HashMap;
use std::hash::Hash;

use crate::shared::sleep_s;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Valve {
    pub id: String,
    pub rate: i64,
    pub leads_to: Vec<String>,
}

impl std::fmt::Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valve id '{}'\n    rate={}\n    leads to: {:?}",
            self.id, self.rate, self.leads_to
        )
    }
}

impl Valve {
    pub fn new(id: &str, rate: i64, leads_to: Vec<&str>) -> Self {
        let others: Vec<String> = leads_to.iter().map(|s| s.to_string()).collect();
        Self {
            id: id.to_string(),
            rate,
            leads_to: others,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Valve>> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (_, (_, id, _, rate, _, _, leads_to)) = tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            cc::i64,
            alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
            alt((tag("valves "), tag("valve "))),
            separated_list0(tag(", "), alpha1),
        ))(line)?;
        valves.insert(id.to_string(), Valve::new(id, rate, leads_to));
    }
    Ok((input, valves))
}

// fn memoize<A, B, F>(f: F, cache: &'static mut HashMap<A, B>) -> impl FnMut(A) -> B
// where
//     A: Eq + Hash + Copy,
//     B: Clone,
//     F: Fn(A) -> B,
// {
//     move |value: A| {
//         let res = cache.entry(value).or_insert_with(|| f(value));
//         res.clone()
//     }
// }

pub fn calculate_max_relief(
    mut opened: String,
    valves: HashMap<String, Valve>,
    mut mins_remaining: i64,
    curr_valve_id: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    // println!(
    //     "Args: opened={opened}, mins_remaining={mins_remaining}, curr_valve_id={curr_valve_id}"
    // );
    if mins_remaining <= 0 {
        return 0;
    }
    let mut most_relief = 0;
    let current_valve = valves.get(curr_valve_id).unwrap();

    for next in &current_valve.leads_to {
        // println!("Current Valve: {current_valve:?}");
        // println!("Next: {next}\n");
        // sleep_s(1, 0);
        opened.push_str(next);
        if !cache.contains_key(&opened) {
            most_relief = std::cmp::max(
                most_relief,
                calculate_max_relief(
                    opened.clone(),
                    valves.clone(),
                    mins_remaining - 1,
                    next,
                    cache,
                ),
            );
            cache.insert(opened.clone(), most_relief);
        }
    }

    if !opened.contains(curr_valve_id) && current_valve.rate > 0 && mins_remaining > 0 {
        // println!("{curr_valve_id} not in opened: {opened}");
        opened.push_str(curr_valve_id);
        mins_remaining -= 1;
        let total_released = mins_remaining * current_valve.rate;

        for next in &current_valve.leads_to {
            // println!("Next: {next} (cur total released: {total_released})");
            // sleep_s(1, 0);
            let mut key = opened.clone();
            key.push_str(next);
            if !cache.contains_key(&key) {
                most_relief = std::cmp::max(
                    most_relief,
                    total_released
                        + calculate_max_relief(
                            key.clone(),
                            valves.clone(),
                            mins_remaining - 1,
                            next,
                            cache,
                        ),
                );
                cache.insert(key, most_relief);
            }
        }
    }
    // println!("Most relief={most_relief}");
    most_relief
}

pub fn run() {
    let input = include_str!("sample_input.txt");
    let valves = parse_input(input).finish().unwrap().1;
    let opened = String::new();
    let mut cache = HashMap::new();
    for (key, valve) in valves.iter() {
        println!("{key}: {valve:?}\n");
    }
    let res = calculate_max_relief(opened, valves, 30, "AA", &mut cache);
    println!("res: {res:?}");
    for (k, v) in cache.iter() {
        println!("k: {k:?}\nvalue: {v}\n");
    }
}
