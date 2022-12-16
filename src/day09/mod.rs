mod parse;

use std::collections::{HashSet, VecDeque};

use parse::{parse_input, GridPos};

use self::parse::Instruction;

const NUM_KNOTS: usize = 10;
const TAIL_IDX: usize = NUM_KNOTS - 1;

struct Simulation {
    instructions: VecDeque<Instruction>,
    knots: [GridPos; NUM_KNOTS],
    first_knot_visited: HashSet<GridPos>, // part 1
    tail_visited: HashSet<GridPos>,       // part 2
}

impl Simulation {
    fn new(path: &str) -> Self {
        let instructions = parse_input(path).expect("error reading input file");

        Self {
            instructions,
            knots: [GridPos { x: 0, y: 0 }; NUM_KNOTS],
            first_knot_visited: HashSet::new(), // part 1
            tail_visited: HashSet::new(),       // part 2
        }
    }

    fn update_state(&mut self) {
        let instruction = match self.instructions.front_mut() {
            Some(instruction) => instruction,
            None => return,
        };
        self.knots[0] += instruction.dir.delta();

        for i in 1..NUM_KNOTS {
            let diff = self.knots[i - 1] - self.knots[i];

            self.knots[i] += get_tail_move(diff);
            // part 1
            match i {
                1 => self.first_knot_visited.insert(self.knots[i]),
                TAIL_IDX => self.tail_visited.insert(self.knots[i]),
                _ => true,
            };
        }
        instruction.dist -= 1;
        if instruction.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

fn get_tail_move(diff: GridPos) -> GridPos {
    let mov = match (diff.x, diff.y) {
        // head on top of tail
        (0, 0) => (0, 0),
        // touching left/up/right/down & diag up-l/up-r/down-r/down-l
        (-1, 0) | (0, 1) | (1, 0) | (0, -1) | (-1, 1) | (1, 1) | (1, -1) | (-1, -1) => (0, 0),
        // needs to move left/up/right/down
        (-2, 0) => (-1, 0),
        (0, 2) => (0, 1),
        (2, 0) => (1, 0),
        (0, -2) => (0, -1),
        // up/right diagonally
        (2, 1) => (1, 1),
        (1, 2) => (1, 1),
        // up/left diagonally
        (-2, 1) => (-1, 1),
        (-1, 2) => (-1, 1),
        // down/right diagonally
        (2, -1) => (1, -1),
        (1, -2) => (1, -1),
        // down/left diagonally
        (-1, -2) => (-1, -1),
        (-2, -1) => (-1, -1),
        // diagonally in general
        (-2, -2) => (-1, -1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (2, 2) => (1, 1),

        _ => panic!("unhandled tail position: {diff:?}"),
    };
    mov.into()
}

pub fn run() {
    println!("\n=== Day 09 ===");
    let mut sim = Simulation::new("src/day09/input.txt");
    while !sim.instructions.is_empty() {
        sim.update_state();
    }
    println!("Part 1: {:?}", sim.first_knot_visited.len());
    println!("Part 2: {:?}", sim.tail_visited.len());
}
