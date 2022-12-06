use std::env::args;

mod shared;

mod day01;
mod day02;
mod day03;
mod day04;

const DAYS: [fn(); 4] = [day01::run, day02::run, day03::run, day04::run];

fn parse_input() {
    let input = args().skip(1).next().expect("error getting arg");
    if input.to_lowercase() == "all".to_string() {
        run_all()
    } else {
        if let Some(day) = input.parse::<isize>().ok() {
            if day <= 0 || day > 25 {
                println!("Not a valid day. Running all days.");
                run_all();
            } else {
                DAYS[day as usize - 1]();
            }
        }
    }
}

fn run_all() {
    for day in DAYS {
        day();
    }
}

fn main() {
    parse_input();
}
