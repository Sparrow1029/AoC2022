use std::{
    cmp::{max, min},
    collections::HashMap,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};

const BOTH_PARTS: bool = false;

type PuzzleData<'a> = (
    Vec<&'a str>,
    HashMap<&'a str, usize>,
    Vec<Vec<usize>>,
    Vec<i32>,
);

fn parse_input(input: &str) -> IResult<&str, PuzzleData> {
    let mut valves: Vec<&str> = Vec::new();
    let mut flows: HashMap<&str, i32> = HashMap::new();
    let mut conn: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut positions: HashMap<&str, usize> = HashMap::new();
    for line in input.lines() {
        let (_, (_, valve_id, _, rate, _, _, leads_to)) = tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            cc::i32,
            alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
            alt((tag("valves "), tag("valve "))),
            separated_list0(tag(", "), alpha1),
        ))(line)?;
        // valves.insert(id.to_string(), Valve::new(id, rate, leads_to));
        valves.push(valve_id);
        flows.insert(valve_id, rate);
        leads_to
            .iter()
            .for_each(|c| conn.entry(valve_id).or_insert(Vec::new()).push(c));
        positions.insert(valve_id, valves.len() - 1);
    }
    let conn_num: Vec<Vec<usize>> = valves
        .iter()
        .map(|v| {
            conn.get(v)
                .unwrap()
                .iter()
                .map(|c| *positions.get(c).unwrap())
                .collect()
        })
        .collect();
    let flows_num: Vec<i32> = valves.iter().map(|v| *flows.get(v).unwrap()).collect();
    Ok((input, (valves, positions, conn_num, flows_num)))
}

fn dfs(
    pos: usize,
    time: i32,
    conn: &Vec<Vec<usize>>,
    flows: &Vec<i32>,
    opened: &mut Vec<bool>,
    cache: &mut HashMap<(usize, i32, Vec<bool>), i32>,
    min_depth: &mut usize,
) -> i32 {
    // Time has run out
    if time <= 0 {
        return 0;
    }
    // All valves are opened, we just wait now
    if opened.iter().all(|v| *v) {
        return 0;
    }
    // Check cache to see if answer is already computed for this state
    if let Some(v) = cache.get(&(pos, time, (*opened).clone())) {
        return *v;
    }
    // Do not open if this valve is already open, or this valve has no flow rate
    let do_not_open = opened[pos] || flows[pos] == 0;
    let if_open: i32;
    if !do_not_open {
        // open this valve...
        opened[pos] = true;
        // Then keep computing the total flow if this valve is opened
        if_open =
            flows[pos] * (time - 1) + dfs(pos, time - 1, conn, flows, opened, cache, min_depth);
        // and re-close it for the next iteration
        opened[pos] = false;
    } else {
        // flow was 0 or this valve was already open, set to min_flow (0)
        if_open = 0;
    }
    // For each connection from this valve, calculate std::cmp::max(total flow `if_open`, recursive dfs total_flow)
    let max_flow = max(
        if_open,
        conn[pos]
            .iter()
            .map(|s| dfs(*s, time - 1, conn, flows, opened, cache, min_depth))
            .max()
            .unwrap(),
    );
    // Insert max_flow result into cache for memoization
    cache.insert((pos, time, (*opened).clone()), max_flow);
    max_flow
}

fn part_1(input: &str) {
    let (valves, positions, conn_num, flows_num) = parse_input(input).finish().unwrap().1;
    // println!(
    //     "valves: {valves:?}\npos: {positions:?}\nconn_num: {conn_num:?}\nflows_num: {flows_num:?}"
    // );
    let mut visited = vec![false; valves.len()];
    let mut cache: HashMap<(usize, i32, Vec<bool>), i32> = HashMap::new();
    let mut min_depth = usize::MAX;
    let max_flow = dfs(
        positions["AA"],
        30,
        &conn_num,
        &flows_num,
        &mut visited,
        &mut cache,
        &mut min_depth,
    );
    println!("Part 1: {max_flow}");
}

/// Turning off for now... too slow
fn part_2(input: &str) {
    let (_, positions, conn_num, flows_num) = parse_input(input).finish().unwrap().1;
    fn dfs(
        one: usize,
        two: usize,
        time: i32,
        conn: &Vec<Vec<usize>>,
        flows: &Vec<i32>,
        opened: u64,
        cache: &mut HashMap<(usize, usize, i32, u64), i32>,
    ) -> i32 {
        if time <= 0 {
            return 0;
        }
        if opened == (1 << (1 + conn.len())) - 1 {
            return 0;
        }
        let my_pos = min(one, two);
        let el_pos = max(one, two);

        if let Some(res) = cache.get(&(my_pos, el_pos, time, opened)) {
            return *res;
        }
        let opened_el = (opened & (1 << el_pos)) != 0;
        let opened_my = (opened & (1 << my_pos)) != 0;
        let me_do_not_open = opened_my || flows[my_pos] == 0;
        let el_do_not_open = opened_el || flows[el_pos] == 0;
        let both_do_not_open = me_do_not_open && el_do_not_open;

        // both open
        let both_open = if me_do_not_open || el_do_not_open || el_pos == my_pos {
            0
        } else {
            let opened = opened | (1 << my_pos) | (1 << el_pos);
            (flows[my_pos] + flows[el_pos]) * (time - 1)
                + dfs(my_pos, el_pos, time - 1, conn, flows, opened, cache)
        };
        let mut max_flow = both_open;

        if !both_do_not_open {
            if el_do_not_open {
                let opened = opened | (1 << my_pos);
                max_flow = max_flow.max(
                    flows[my_pos] * (time - 1)
                        + conn[el_pos]
                            .iter()
                            .map(|el_next_pos| {
                                dfs(my_pos, *el_next_pos, time - 1, conn, flows, opened, cache)
                            })
                            .max()
                            .unwrap(),
                );
            } else if me_do_not_open {
                let opened = opened | (1 << el_pos);
                max_flow = max_flow.max(
                    flows[el_pos] * (time - 1)
                        + conn[my_pos]
                            .iter()
                            .map(|my_next_pos| {
                                dfs(*my_next_pos, el_pos, time - 1, conn, flows, opened, cache)
                            })
                            .max()
                            .unwrap(),
                );
            } else {
                let opened_1 = opened | (1 << my_pos);
                max_flow = max_flow.max(
                    flows[my_pos] * (time - 1)
                        + conn[el_pos]
                            .iter()
                            .map(|el_next_pos| {
                                dfs(my_pos, *el_next_pos, time - 1, conn, flows, opened_1, cache)
                            })
                            .max()
                            .unwrap(),
                );
                let opened_2 = opened | (1 << el_pos);
                max_flow = max_flow.max(
                    flows[el_pos] * (time - 1)
                        + conn[my_pos]
                            .iter()
                            .map(|my_next_pos| {
                                dfs(*my_next_pos, el_pos, time - 1, conn, flows, opened_2, cache)
                            })
                            .max()
                            .unwrap(),
                );
            }
        }
        if both_open == 0 {
            let both_go = conn[my_pos]
                .iter()
                .map(|my_next_pos| {
                    conn[el_pos]
                        .iter()
                        .map(|el_next_pos| {
                            dfs(
                                *my_next_pos,
                                *el_next_pos,
                                time - 1,
                                conn,
                                flows,
                                opened,
                                cache,
                            )
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap();
            max_flow = max_flow.max(both_go);
        }

        cache.insert((my_pos, el_pos, time, opened), max_flow);
        max_flow
    }

    let mut cache: HashMap<(usize, usize, i32, u64), i32> = HashMap::new();
    let max_flow = dfs(
        positions["AA"],
        positions["AA"],
        26,
        &conn_num,
        &flows_num,
        0,
        &mut cache,
    );
    println!("Part 2: {max_flow}");
}

pub fn run() {
    println!("\n=== Day 16 ===");
    let input = include_str!("input.txt");
    part_1(input);
    if BOTH_PARTS {
        part_2(input);
    }
}
