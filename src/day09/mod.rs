mod parse;

use std::collections::{HashSet, VecDeque};

use parse::{parse_input, GridPos};

use self::parse::Instruction;

struct Simulation {
    instructions: VecDeque<Instruction>,
    head: GridPos,
    tail: GridPos,
    tail_visited: HashSet<GridPos>,
}

impl Simulation {
    fn new(path: &str) -> Self {
        let instructions =
            parse_input("src/day09/sample_input.txt").expect("error reading input file");

        Self {
            instructions,
            head: GridPos { x: 0, y: 0 },
            tail: GridPos { x: 0, y: 0 },
            tail_visited: HashSet::new(),
        }
    }

    fn update_state(&mut self) {
        let ins = match self.instructions.front_mut() {
            Some(instruction) => instruction,
            None => return,
        };
        self.head += ins.dir.delta();

        let diff = self.head - self.tail;
    }
}

fn get_tail_move(diff: GridPos) -> (i32, i32) {
    match (diff.x, diff.y) {
        (0, 0) => (0, 0),
        // touching left/up/right/down & diag up-l/up-r/down-r/down-l
        (-1, 0) | (0, 1) | (1, 0) | (0, -1) | (-1, 1) | (1, 1) | (1, -1) | (-1, -1) => (0, 0),
        _ => panic!("unhandled tail position"),
    }
}

pub fn run() {}
