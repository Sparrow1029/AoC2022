#![feature(iter_array_chunks)]
use std::env::args;

mod shared;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

const DAYS: [fn(); 7] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
];

fn parse_input() {
    let input = args().nth(1).expect("error getting arg");
    if input.to_lowercase() == "all" {
        run_all()
    } else if let Ok(day) = input.parse::<isize>() {
        if day <= 0 || day > 25 {
            println!("Not a valid day. Running all days.");
            run_all();
        } else {
            DAYS[day as usize - 1]();
        }
    }
}

fn run_all() {
    for day in DAYS {
        day();
    }
}

fn main() {
    color_eyre::install().unwrap();
    parse_input();
}
