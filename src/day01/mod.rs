use crate::shared::read_lines;

pub fn run() {
    println!("=== Day 01 ===");

    let input_lines = read_lines("src/day01/input.txt").expect("Couldn't read file");

    let mut calorie_totals: Vec<usize> = Vec::new();
    let mut total = 0;
    let mut cur_max = 0;

    for line in input_lines {
        if let Ok(val) = line {
            match val.trim().is_empty() {
                true => {
                    if total > cur_max {
                        cur_max = total;
                    }
                    calorie_totals.push(total);
                    total = 0;
                    continue;
                }
                false => {
                    total += val.trim().parse::<usize>().expect("not a number");
                }
            }
        }
    }
    calorie_totals.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", cur_max);
    println!("Part 2: {}", calorie_totals[0..3].iter().sum::<usize>());
}
