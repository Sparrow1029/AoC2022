use crate::shared::read_lines;
use regex;

type Stacks = Vec<Stack>;

#[derive(Debug, Copy, Clone)]
struct Stack {
    crates: [u8; 56],
    top_idx: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            crates: [0; 56],
            top_idx: 0,
        }
    }
    fn stack_on_top() {}
    fn add(&mut self, item: u8) {
        self.top_idx += 1;
        self.crates[self.top_idx] = item;
    }
}

fn parse_input() -> Stacks {
    let lines = read_lines("src/day05/input.txt")
        .expect("error reading input file")
        .map(|l| l.expect("error reading line"));

    let re = regex::Regex::new(r"\[|\]\s|\]").unwrap();
    let mut cur_line = 1;
    for line in lines {
        if cur_line < 9 {
            let data = line.bytes();
            for i in [0..]
                let crate_ = slice.take(3).filter(|b| ![91, 93].contains(b));
            }
            println!("{data:?}");
            cur_line += 1;
        } else {
            break;
        }
    }
    let mut stacks: Stacks = vec![Stack::new(); 9];
    stacks
}

pub fn run() {
    parse_input();
}
