/// This method works for the sample input, but I guess the actual puzzle input was more complex
/// Basic Idea is instead of representing a Tetris grid as x, y coords (because part2 requires 1_000_000_000_000
/// pieces falling), we look at all the existing heights of the pieces from _top-down_ view, and decrement/move current
/// y-values (or heights) in one [usize; 7] array, keeping track of the current state, and a separate array with the
/// floating piece.
///
/// example:
/// [0011110]  >  [0067610]  >  [0676110]  >  [0565110] ... [0034310]
/// |.......|7    |...@...|7    |..@....|7    |..v....|7    |.......|7
/// |.......|6    |.<@@@..|6    |.@@@...|6    |..@....|6    |.......|6
/// |.......|5    |...@...|5    |..@....|5    |.@@@...|5    |.......|5
/// |.......|4    |.......|4    |.......|4    |..@....|4    |...#...|4
/// |.......|3    |.......|3    |.......|3    |.......|3    |..###..|3
/// |.......|2    |.......|2    |.......|2    |.......|2    |...#...|2
/// |..####.|1    |..####.|1    |..####.|1    |..####.|1    |..####.|1
/// +-------+     +-------+     +-------+     +-------+     +-------+
/// I saw the issue immediately, how do you tell when a piece slides under another piece?
/// example:
/// [7714544]  >  [0334544]  >  [0334544]
/// |@@.....|7   <|.......|7    |.......|7
/// |@@.....|6   <|.......|6    |.......|6
/// |....#..|5   <|....#..|5    |....#..|5
/// |...####|4   <|...####|4    |...####|4
/// |....#.#|3   >|.@@?#.#|3    |..@@#.#|3 < algorithm doesn't know this space is empty
/// |....###|2   v|.@@?###|2    |..@@###|2 < maybe could keep some kind of index of known, reachable gaps, but that's real complex...
/// |..####.|1    |..####.|1    |..####.|1
/// +-------+     +-------+     +-------+
/// I was just hoping that the puzzle input wouldn't have any of that going on... so maybe the sample_input just
/// randomly worked by complete fluke... I don't know
/// My answer for part 1 wasn't too far off though ;)
///
/// I'll have to come back to this, and try to implement a real tetris-y solution, where I clear the board every so often
/// only keeping track of the state from the appropriate current height
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Line,
    Cross,
    Elbow,
    Pole,
    Box,
}

pub fn pieces() -> Vec<Piece> {
    vec![
        Piece {
            shape: Shape::Line,
            y_vals: [0, 0, 1, 1, 1, 1, 0],
            left_x: 2,
            len: 4,
        },
        Piece {
            shape: Shape::Cross,
            y_vals: [0, 0, 2, 3, 2, 0, 0],
            left_x: 2,
            len: 3,
        },
        Piece {
            shape: Shape::Elbow,
            y_vals: [0, 0, 1, 1, 3, 0, 0],
            left_x: 2,
            len: 3,
        },
        Piece {
            shape: Shape::Pole,
            y_vals: [0, 0, 4, 0, 0, 0, 0],
            left_x: 2,
            len: 1,
        },
        Piece {
            shape: Shape::Box,
            y_vals: [0, 0, 2, 2, 0, 0, 0],
            left_x: 2,
            len: 2,
        },
    ]
}

pub struct Piece {
    pub shape: Shape,
    pub y_vals: [usize; 7],
    pub left_x: usize,
    pub len: usize,
}

impl Piece {
    /// Look at the pieces from top-down
    pub fn fall(&mut self, field: &mut [usize; 7]) {
        for i in self.left_x..self.left_x + self.len {
            field[i] -= 1;
        }
    }

    pub fn slide(
        &mut self,
        direction: Direction,
        next_state: &mut [usize; 7],
        orig_state: &[usize; 7],
    ) {
        let prev_left_idx = self.left_x;
        let prev_heights = &(next_state.clone()[prev_left_idx..prev_left_idx + self.len]);
        let prev_right_idx = self.left_x + self.len - 1;
        match direction {
            Direction::Left => {
                self.left_x = self.left_x.saturating_sub(1);
                if self.left_x < prev_left_idx {
                    self.y_vals.rotate_left(1);
                    for i in 0..self.len {
                        next_state[self.left_x + i] = prev_heights[i]
                    }
                    next_state[prev_right_idx] = orig_state[prev_right_idx];
                }
            }
            Direction::Right => {
                if self.left_x + self.len == 7 {
                    // don't move
                } else {
                    // println!("prev_heights: {prev_heights:?}");
                    self.left_x += 1;
                    self.y_vals.rotate_right(1);
                    for i in 0..self.len {
                        next_state[self.left_x + i] = prev_heights[i]
                    }
                    next_state[prev_left_idx] = orig_state[prev_left_idx];
                }
            }
        };
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.y_vals)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct State {
    field: [usize; 7],
    total_pieces: usize,
    directions: Vec<Direction>,
    cur_dir_idx: usize,
    cur_height: usize,
}

impl State {
    fn new(directions: Vec<Direction>) -> Self {
        Self {
            field: [0; 7],
            total_pieces: 0,
            directions,
            cur_dir_idx: 0,
            cur_height: 0,
        }
    }

    fn check_piece_can_slide(
        &self,
        piece: &Piece,
        direction: Direction,
        next_state: &[usize; 7],
        orig_state: &[usize; 7],
    ) -> bool {
        match direction {
            Direction::Right => {
                let rx = piece.left_x + piece.len - 1;
                if rx == 6 {
                    // already at right wall
                    return false;
                }
                match piece.shape {
                    // @@@@#.. <- check here
                    // ....#..
                    Shape::Line => next_state[rx] > orig_state[rx + 1],
                    // ..@....
                    // .@@@#.. <- check here
                    // ..@###. <- and here
                    // ....#..
                    Shape::Cross => {
                        next_state[rx] > orig_state[rx + 1]
                            && (next_state[rx - 1] - 2) > orig_state[rx]
                    }
                    // ..@....    ...@...    ..@@...
                    // ..@....    ...@...    ..@@#.. <-
                    // @@@##.# <- ...@...    ....#..
                    // ..#####    ...@#.# <- ....#..
                    // .####.#    ....###    ....#..

                    // Shape::Elbow | Shape::Pole | Shape::Box => (0..piece.y_vals[rx])
                    //     .all(|y| next_state[rx] - y > orig_state[rx + 1]),
                    Shape::Elbow | Shape::Pole | Shape::Box => {
                        next_state[rx] - (piece.y_vals[rx] - 1) > orig_state[rx + 1]
                    }
                }
            }
            Direction::Left => {
                let lx = piece.left_x;
                if lx == 0 {
                    // already at left wall
                    return false;
                }
                match piece.shape {
                    // ..#@@@@ <- check here
                    // ..#.#..
                    Shape::Line => next_state[lx] > orig_state[lx - 1],
                    // ....@..
                    // ..#@@@. <- check here
                    // .###@.. <- and here
                    // ..#....
                    Shape::Cross => {
                        next_state[lx] > orig_state[lx - 1]
                            && next_state[lx + 1] - 2 > orig_state[lx]
                    }
                    // Shape::Elbow | Shape::Pole | Shape::Box => {
                    //     (0..piece.y_vals[lx]).all(|y| next_state[lx] - y > orig_state[lx - 1])
                    // }
                    // .....@.    ...@...    ..@@...
                    // .....@.    ...@...    ..@@#.. <-
                    // ..#@@@. <- ...@...    ....#..
                    // ..#####    ...@#.# <- ....#..
                    // .####.#    ....###    ....#..
                    Shape::Elbow | Shape::Pole | Shape::Box => {
                        next_state[lx] - (piece.y_vals[lx] - 1) > orig_state[lx - 1]
                    }
                }
            }
        }
    }

    fn check_piece_stop_falling(&self, piece: &Piece, next_state: &[usize; 7]) -> bool {
        let mut test = next_state.clone();
        for i in 0..7 {
            test[i] -= piece.y_vals[i];
        }
        match piece.shape {
            Shape::Cross => {
                let lx = piece.left_x;
                // bottom piece
                // |...@...| 4
                // |.#@@@..| 3
                // |###@##.| 2 <- check bottom, then either side
                // |.#..##.| 1
                // +_______+
                if next_state[lx + 1] - 3 == self.field[lx + 1]
                    || next_state[lx + 1] - 2 == self.field[lx]
                    || next_state[lx + 1] - 2 == self.field[lx + 2]
                {
                    return true;
                }
                // left
                if next_state[lx] - 1 == self.field[lx] {
                    return true;
                }
                // right
                if next_state[lx + 2] - 1 == self.field[lx + 2] {
                    return true;
                }
            }
            _ => {
                for i in piece.left_x..piece.left_x + piece.len {
                    if next_state[i] - piece.y_vals[i] == self.field[i] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn run_sim(&mut self, num_rocks_to_fall: usize) {
        'outer: loop {
            for mut piece in pieces() {
                if self.total_pieces == num_rocks_to_fall {
                    break 'outer;
                }
                // println!("Current state: {:?}", self.field);
                // print!("Shape: {:?} -> [", piece.shape);
                let mut next_state = self.field.clone();
                for i in 0..7 {
                    // start 3 spaces up from cur top
                    if piece.y_vals[i] > 0 {
                        next_state[i] = piece.y_vals[i] + self.cur_height + 3;
                    }
                }
                loop {
                    let direction = self.directions[self.cur_dir_idx % self.directions.len()];
                    // print!("{direction:?}, ");
                    self.cur_dir_idx += 1;

                    if self.check_piece_can_slide(&piece, direction, &next_state, &self.field) {
                        piece.slide(direction, &mut next_state, &self.field);
                    }
                    if self.check_piece_stop_falling(&piece, &next_state) {
                        break;
                    }
                    piece.fall(&mut next_state);
                }
                // print!("\x08\x08 \x08]\n   New state:  {:?}\n\n", next_state);
                self.total_pieces += 1;
                self.field = next_state;
                self.cur_height = *(self.field.iter().max().unwrap());
            }
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect()
}

pub fn run() {
    println!("\n=== Day 17 ===");
    let input = include_str!("sample_input.txt");
    let directions = parse(input);
    let mut state = State::new(directions);
    state.run_sim(2022);
    // println!("Part 1: {}", state.cur_height);
    println!("Part 1: TODO!");
    println!("Part 2: TODO!");
}
