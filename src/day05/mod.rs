use crate::shared::read_lines;

const NUM_STACKS: usize = 9;
const START_CRATES: usize = 8;
const MAX_CRATES: usize = NUM_STACKS * START_CRATES;

type Stacks = Vec<Stack>;

#[derive(Debug, Copy, Clone)]
struct Stack {
    crates: [u8; MAX_CRATES],
    top_idx: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            crates: [0; MAX_CRATES],
            top_idx: START_CRATES - 1, // index
        }
    }
    fn stack_on_top() {}

    fn add(&mut self, item: u8) {
        self.top_idx -= 1;
        self.crates[self.top_idx] = item;
    }

    fn reset_top_idx(&mut self) {
        for (i, val) in self.crates.iter().enumerate() {
            if *val == 0u8 {
                self.top_idx = i - 1;
                break;
            }
        }
    }
}

fn parse_input() -> Stacks {
    let lines = read_lines("src/day05/input.txt")
        .expect("error reading input file")
        .map(|l| l.expect("error reading line"));

    let mut stacks: Stacks = vec![Stack::new(); 9];

    let mut cur_line = 1;
    for line in lines {
        if cur_line < 9 {
            let mut iter = line.bytes().array_chunks::<4>();
            let mut cur_stack_idx = 0;
            for [_, c, _, _] in &mut iter {
                match c {
                    b' ' => stacks[cur_stack_idx].add(0),
                    _ => stacks[cur_stack_idx].add(c),
                }
                cur_stack_idx += 1
            }
            let last_val = iter.into_remainder().unwrap().nth(1).unwrap();
            stacks[cur_stack_idx].add(last_val);
            cur_line += 1;
        }
        for stack in &stacks {
            println!("{stack:?}");
        }
    }
    stacks
}

pub fn run() {
    parse_input();
}
