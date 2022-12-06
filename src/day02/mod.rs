use crate::shared::read_lines;

const CHOICE_SCORES: [usize; 3] = [1, 2, 3];
const WIN_PAIRS: [(char, char); 3] = [('A', 'Y'), ('B', 'Z'), ('C', 'X')];
const LOSS_PAIRS: [(char, char); 3] = [('A', 'Z'), ('B', 'X'), ('C', 'Y')];
const WIN_CHOICES: [usize; 3] = [2, 3, 1];
const LOSS_CHOICES: [usize; 3] = [3, 1, 2];

fn get_round_score_pt1(round: &(char, char)) -> usize {
    let choice_score = CHOICE_SCORES[round.1 as usize - 'X' as usize];
    let game_score = if WIN_PAIRS.contains(round) {
        6
    } else if LOSS_PAIRS.contains(round) {
        0
    } else {
        3
    };
    choice_score + game_score
}

fn get_round_score_pt2(round: &(char, char)) -> usize {
    let opponent_choice = round.0 as usize - 'A' as usize;
    let win_val = match round.1 {
        'X' => LOSS_CHOICES[opponent_choice],
        'Y' => opponent_choice + 1 + 3, // Rock, paper, scissors are +1 bc of indexing
        'Z' => WIN_CHOICES[opponent_choice] + 6,
        _ => unreachable!(),
    };
    win_val
}

pub fn run() {
    println!("\n=== Day 02 ===");
    let input = read_lines("src/day02/input.txt")
        .expect("error reading input file")
        .map(|l| {
            let line = l.expect("error reading line");
            (
                line.chars().nth(0).expect("not a char"),
                line.chars().nth(2).expect("not a char"),
            )
        })
        .collect::<Vec<(char, char)>>();

    let part1_score: usize = input.iter().map(|round| get_round_score_pt1(round)).sum();
    println!("Part 1: {part1_score}");

    let part2_score: usize = input.iter().map(|round| get_round_score_pt2(round)).sum();
    println!("Part 2: {part2_score}");
}
