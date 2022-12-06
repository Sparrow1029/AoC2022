use std::env::args;

mod shared;

mod day01;
mod day02;
mod day03;

const DAYS: [fn(); 3] = [day01::run, day02::run, day03::run];

fn parse_input() {
    let input = args().skip(1).next().expect("error getting arg");
    if input == "all".to_string() {
        run_all()
    } else {
        if let Some(day) = input.parse::<usize>().ok() {
            if day <= 0 || day > 25 {
                println!("Not a valid day. Running all days.");
                run_all();
            }
            DAYS[day - 1]();
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
