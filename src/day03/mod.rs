use crate::shared::read_lines;
use std::collections::HashSet;

pub fn run() {
    println!("\n=== Day 03 ===");
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<String>) -> usize {
    let mut total = 0;
    for rucksack in input {
        total += get_dup_item_priority_from_rucksack(rucksack) as usize
    }
    total
}

fn part2(input: &Vec<String>) -> usize {
    let mut total = 0;
    for group in input.chunks(3) {
        total += get_shared_item_priority(group) as usize;
    }
    total
}

fn parse_input() -> Vec<String> {
    read_lines("src/day03/input.txt")
        .expect("Error reading input file")
        .map(|line| line.expect("error reading line").trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn get_shared_item_priority(rucksacks: &[String]) -> u8 {
    let (first, rest) = rucksacks.split_at(1);
    let common = first[0]
        .chars()
        .filter(|c| rest[0].contains(*c) && rest[1].contains(*c))
        .next()
        .expect("No common item found");
    get_priority(&common)
}

fn get_dup_item_priority_from_rucksack(rucksack: &String) -> u8 {
    let (s1, s2) = rucksack.split_at(rucksack.len() / 2);
    let (set1, set2) = (
        s1.chars().collect::<HashSet<char>>(),
        s2.chars().collect::<HashSet<char>>(),
    );
    let mut intersection = set1.intersection(&set2);
    let ret = get_priority(intersection.next().unwrap());
    ret
}

fn get_priority(c: &char) -> u8 {
    let val = *c as u8;
    match val <= 90 {
        true => val - 38,  // A-Z
        false => val - 96, // a-z
    }
}
