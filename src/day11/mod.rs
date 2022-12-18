/// Fully adapted from fasterthanli.me's solution:
/// https://fasterthanli.me/series/advent-of-code-2022/part-11
mod shared;
use shared::{parse_monkey, Monkey};

pub(crate) fn run() {
    println!("\n=== Day 11 ===");
    let chunks = include_str!("input.txt").split("\n\n");
    let mut monkeys = vec![];
    for chunk in chunks {
        monkeys.push(parse_monkey(chunk).unwrap().1);
    }

    part1(monkeys.clone());
    part2(monkeys);
}

fn monkey_around(monkeys: &mut [Monkey], rounds: u64, worried: bool) {
    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut cur_monkey = monkeys[i].clone();
            cur_monkey.num_items_inspected += cur_monkey.items.len() as u64;
            while let Some(mut item) = cur_monkey.items.pop_front() {
                item %= divisor_product;
                let mut new_worry = monkeys[i].operation.eval(item);
                if !worried {
                    new_worry /= 3;
                };
                if new_worry % cur_monkey.divisor == 0 {
                    monkeys[cur_monkey.receivers.0].items.push_back(new_worry)
                } else {
                    monkeys[cur_monkey.receivers.1].items.push_back(new_worry)
                }
            }
            cur_monkey.items.clear();
            monkeys[i] = cur_monkey;
        }
    }
}

fn monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut inspected = monkeys
        .iter()
        .map(|m| m.num_items_inspected)
        .collect::<Vec<u64>>();
    inspected.sort_by(|a, b| b.cmp(a));
    inspected[0] * inspected[1]
}

fn part1(mut monkeys: Vec<Monkey>) {
    monkey_around(&mut monkeys, 20, false);
    println!("Part 1: {}", monkey_business(&monkeys));
}

fn part2(mut monkeys: Vec<Monkey>) {
    monkey_around(&mut monkeys, 10_000, true);
    println!("Part 2: {}", monkey_business(&monkeys));
}
