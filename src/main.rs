#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(slice_flatten)]
use std::env::args;

mod shared;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const DAYS: [fn(); 25] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day23::run,
    day24::run,
    day25::run,
];

fn parse_input() {
    if let Some(input) = args().nth(1) {
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
    } else {
        run_all()
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
