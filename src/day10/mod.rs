use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    sequence::preceded,
    Finish, IResult,
};

use crate::shared::read_lines;

const TARGET_CYCLES_PT1: [u32; 6] = [20, 60, 100, 140, 180, 220];

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
}

#[derive(Debug)]
struct StateMachine {
    instructions: VecDeque<Instruction>,
    rx: i32,
    cycle: u32,
    signal_strengths: Vec<i32>,
    display: [u8; 240],
}

impl std::fmt::Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cycle: {}, rx: {}", self.cycle, self.rx)
    }
}

impl StateMachine {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            instructions,
            rx: 1,
            cycle: 0,
            signal_strengths: Vec::new(),
            display: [b'.'; 240],
        }
    }

    fn process(&mut self) {
        while let Some(instruction) = self.instructions.pop_front() {
            self.cycle += 1;
            self.check_cycle();

            match instruction {
                Instruction::Addx(val) => {
                    self.cycle += 1;
                    self.check_cycle();

                    self.rx += val;
                }
                Instruction::Noop => {}
            };
        }
    }

    fn check_cycle(&mut self) {
        if TARGET_CYCLES_PT1.contains(&self.cycle) {
            self.signal_strengths.push(self.cycle as i32 * self.rx);
        }
        self.draw_to_crt()
    }

    fn draw_to_crt(&mut self) {
        let cur_pix = (self.cycle - 1) % 40;
        let sprite_pos = (self.rx - 1)..=(self.rx + 1);
        match sprite_pos.contains(&(cur_pix as i32)) {
            true => self.display[self.cycle as usize - 1] = b'#',
            false => {}
        }
    }

    fn show(&self) {
        for crt_line in self.display.array_chunks::<40>() {
            crt_line.iter().for_each(|c| print!("{} ", char::from(*c)));
            println!();
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
    let mut machine = StateMachine::new(instructions.clone());
    machine.process();
    println!("Part 1: {}", machine.signal_strengths.iter().sum::<i32>());

    let mut display = StateMachine::new(instructions);
    display.process();
    println!("Part 2:");
    display.show();
}
