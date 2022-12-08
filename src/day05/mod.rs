use crate::shared::read_lines;

const NUM_STACKS: usize = 9;
const START_CRATES: usize = 8;
const MAX_CRATES: usize = NUM_STACKS * START_CRATES;

type Stacks = [[u8; MAX_CRATES]; NUM_STACKS];
type Move = (usize, usize, usize);

/// Take the first 8 lines of the input, and read them as bytes
/// to be inserted into arrays
#[allow(clippy::needless_range_loop)]
fn populate_stacks(stacks: &mut Stacks) {
    let lines = read_lines("src/day05/input.txt")
        .expect("error reading input file")
        .map(|l| l.expect("error reading line"));

    let mut cur_top_pos = 8;
    for line in lines.take(8) {
        cur_top_pos -= 1;
        let mut iter = line.bytes().skip(1);
        stacks[0][cur_top_pos] = iter.next().unwrap();
        for i in 1..9 {
            if let Some(val) = iter.nth(3) {
                match val {
                    32 => continue,
                    _ => stacks[i][cur_top_pos] = val,
                }
            }
        }
    }
}

/// For the rest of the input, output a vector of tuples containing
/// (number of crates to move, stack to move from, stack to move to)
fn get_moves() -> Vec<Move> {
    let mut moves = vec![];
    read_lines("src/day05/input.txt")
        .expect("error reading input file")
        .map(|l| l.expect("error reading line"))
        .skip(10)
        .for_each(|line| {
            let values = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if let [amt, from, to] = values[..] {
                moves.push((amt, from - 1, to - 1));
            }
        });
    moves
}

/// Get index of the first '0' value in one of the crate stacks
fn get_top_idx(stack: &[u8]) -> usize {
    stack.iter().position(|v| *v == 0).unwrap()
}

/// Move values around in the existing crate arrays
/// assign a vector to hold the values in the interim
fn move_crates(stacks: &mut Stacks, move_cmd: Move, part: usize) {
    let (amt, from, to) = move_cmd;
    let top_idx_from = get_top_idx(&stacks[from]);
    let top_idx_to = get_top_idx(&stacks[to]);

    let mut crates_to_move: Vec<u8> = vec![];
    if part == 1 {
        for i in ((top_idx_from - amt)..top_idx_from).rev() {
            crates_to_move.push(stacks[from][i]);
            stacks[from][i] = 0;
        }
    } else if part == 2 {
        for i in top_idx_from - amt..top_idx_from {
            crates_to_move.push(stacks[from][i]);
            stacks[from][i] = 0;
        }
    }
    for (i, val) in crates_to_move.iter().enumerate() {
        stacks[to][top_idx_to + i] = *val;
    }
}

/// Get all the u8 values of the top crates for each stack,
/// convert them to `char`s and collect them into a `String`
fn get_top_crates(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|arr| arr[get_top_idx(arr) - 1] as char)
        .collect::<String>()
}

pub fn part1() {
    let mut stacks = [[0; MAX_CRATES]; NUM_STACKS];
    populate_stacks(&mut stacks);
    let moves = get_moves();
    for move_cmd in moves {
        move_crates(&mut stacks, move_cmd, 1);
    }

    println!("Part 1: {}", get_top_crates(&stacks));
}

pub fn part2() {
    let mut stacks = [[0; MAX_CRATES]; NUM_STACKS];
    populate_stacks(&mut stacks);
    let moves = get_moves();
    for move_cmd in moves {
        move_crates(&mut stacks, move_cmd, 2);
    }

    println!("Part 2: {}", get_top_crates(&stacks));
}

pub fn run() {
    println!("\n=== Day 05 ===");
    part1();
    part2();
}
