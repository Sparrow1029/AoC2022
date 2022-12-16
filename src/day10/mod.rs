use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    sequence::preceded,
    Finish, IResult,
};

use crate::shared::read_lines;

const TARGET_CYCLES_PT1: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);
        alt((value(Self::Noop, noop), map(addx, Self::Addx)))(i)
    }

    // fn cycles(self) -> u32 {
    //     match self {
    //         Self::Noop => 1,
    //         Self::Addx(_) => 2,
    //     }
    // }
}

#[derive(Debug)]
struct StateMachine {
    instructions: VecDeque<Instruction>,
    rx: i32,
    rb: i32,
    cycle: i32,
    signal_strengths: Vec<i32>,
}

impl std::fmt::Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cycle: {}, rx: {}, rb: {}", self.cycle, self.rx, self.rb)
    }
}

impl StateMachine {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            instructions,
            rx: 1,
            rb: 0,
            cycle: 1,
            signal_strengths: Vec::new(),
        }
    }

    fn check_cycle(&mut self) {
        // println!("{self}\n");
        if TARGET_CYCLES_PT1.contains(&self.cycle) {
            self.signal_strengths.push(self.cycle * self.rx);
            // println!(
            //     "target cycle '{}' reached. RX val: {}. Signal strengths: {:?}",
            //     self.cycle, self.rx, self.signal_strengths
            // );
        }
    }

    fn process(&mut self) {
        while let Some(instruction) = self.instructions.pop_front() {
            // println!("\nCurrent ins: {instruction:?}");
            self.check_cycle();
            self.rb = 0;
            match instruction {
                Instruction::Addx(val) => {
                    self.rb = val;
                    self.cycle += 1;
                    self.check_cycle();
                    self.rx += self.rb;
                }
                Instruction::Noop => {}
            };
            self.cycle += 1;
        }
    }
}

fn parse_input(path: &str) -> Result<VecDeque<Instruction>, std::io::Error> {
    let input_lines = read_lines(path).expect("error reading input file");
    Ok(input_lines
        .map(|l| {
            if let Ok(line) = l {
                all_consuming(Instruction::parse)(line.as_str())
                    .finish()
                    .unwrap()
                    .1
            } else {
                panic!("error parsing instruction from line")
            }
        })
        .collect())
}

pub fn run() {
    println!("\n=== Day 10 ===");
    let instructions = parse_input("src/day10/input.txt").expect("error parsing input");
    let mut machine = StateMachine::new(instructions);
    machine.process();
    println!("Part 1: {}", machine.signal_strengths.iter().sum::<i32>());
}
