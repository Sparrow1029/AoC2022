use std::collections::HashMap;

#[derive(Debug)]
enum Monkey<'a> {
    Num(isize),
    Calculated(Op, &'a str, &'a str),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Sub,
    Div,
}

fn calc_monkey(name: &str, monkeys: &HashMap<&str, Monkey>) -> isize {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Calculated(op, lhs, rhs) => {
            let lhs_num = calc_monkey(lhs, monkeys);
            let rhs_num = calc_monkey(rhs, monkeys);
            match op {
                Op::Add => lhs_num + rhs_num,
                Op::Sub => lhs_num - rhs_num,
                Op::Mul => lhs_num * rhs_num,
                Op::Div => lhs_num / rhs_num,
            }
        }
    }
}

fn parse<'a>() -> HashMap<&'a str, Monkey<'a>> {
    let input = include_str!("input.txt");
    let mut monkeys = HashMap::new();
    for line in input.trim().lines() {
        let all = line.split_ascii_whitespace().collect::<Vec<&str>>();
        match all.len() {
            2 => {
                monkeys.insert(
                    all[0].strip_suffix(':').unwrap(),
                    Monkey::Num(all[1].parse::<isize>().unwrap()),
                );
            }
            4 => {
                let name = all[0].strip_suffix(':').unwrap();
                let op = match all[2] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!("Invalid mathematical operator"),
                };
                monkeys.insert(name, Monkey::Calculated(op, all[1], all[3]));
            }
            _ => unreachable!(),
        }
    }
    monkeys
}

fn needs_calc_human(name: &str, monkeys: &HashMap<&str, Monkey>) -> bool {
    if name == "humn" {
        return true;
    }
    match monkeys[name] {
        Monkey::Num(_) => false,
        Monkey::Calculated(_, lhs, rhs) => {
            needs_calc_human(lhs, monkeys) || needs_calc_human(rhs, monkeys)
        }
    }
}

fn calc_human(name: &str, value: isize, monkeys: &HashMap<&str, Monkey>) -> isize {
    if name == "humn" {
        return value;
    }

    match monkeys[name] {
        Monkey::Num(n) => n,
        Monkey::Calculated(op, lhs, rhs) => {
            let (new_name, new_value) = if needs_calc_human(lhs, monkeys) {
                let rhs_num = calc_monkey(rhs, monkeys);
                // Flip the operations to solve for unknown side
                let new_value = match op {
                    Op::Add => value - rhs_num,
                    Op::Sub => value + rhs_num,
                    Op::Mul => value / rhs_num,
                    Op::Div => value * rhs_num,
                };
                (lhs, new_value)
            } else {
                let lhs_num = calc_monkey(lhs, monkeys);
                let new_value = match op {
                    Op::Add => value - lhs_num,
                    Op::Sub => lhs_num - value,
                    Op::Mul => value / lhs_num,
                    Op::Div => lhs_num / value,
                };
                (rhs, new_value)
            };
            calc_human(new_name, new_value, monkeys)
        }
    }
}

fn part_1() -> isize {
    let monkeys = parse();
    calc_monkey("root", &monkeys)
}

fn part_2() -> isize {
    let monkeys = parse();
    let Monkey::Calculated(_, lhs, rhs) = monkeys["root"] else {
        panic!("Root monkey has to be calculated")
    };
    let (name, value) = if needs_calc_human(lhs, &monkeys) {
        let rhs_num = calc_monkey(rhs, &monkeys);
        (lhs, rhs_num)
    } else {
        let lhs_num = calc_monkey(lhs, &monkeys);
        (rhs, lhs_num)
    };
    calc_human(name, value, &monkeys)
}

pub fn run() {
    println!("\n=== Day 21 ===");
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
