use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Op<'a> {
    Mul(&'a str, &'a str),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Div(&'a str, &'a str),
}

pub fn parse<'a>() -> (HashMap<&'a str, usize>, HashMap<&'a str, Op<'a>>) {
    let input = include_str!("sample_input.txt");
    let mut values = HashMap::new();
    let mut unknown = HashMap::new();
    for line in input.trim().lines() {
        let all = line.split_ascii_whitespace().collect::<Vec<&str>>();
        match all.len() {
            2 => {
                values.insert(
                    all[0].strip_suffix(':').unwrap(),
                    all[1].parse::<usize>().unwrap(),
                );
            }
            4 => {
                let key = all[0].strip_suffix(':').unwrap();
                let value = match all[2] {
                    "+" => Op::Add(all[1], all[3]),
                    "-" => Op::Sub(all[1], all[3]),
                    "*" => Op::Mul(all[1], all[3]),
                    "/" => Op::Div(all[1], all[3]),
                    _ => unreachable!(),
                };
                unknown.insert(key, value);
            }
            _ => unreachable!(),
        }
    }
    (values, unknown)
}

pub fn run() {
    let (values, unknown) = parse();
    println!("Values:");
    for (k, v) in values.iter() {
        println!("  {k:?} -> {v:?}");
    }
    println!("\n\nUnknown:");
    for (k, v) in unknown.iter() {
        println!("  {k:?} -> {v:?}");
    }
}
