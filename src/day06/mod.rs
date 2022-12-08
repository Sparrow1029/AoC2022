use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day06/input.txt").expect("failed to open file");
    println!("Part 1: {}", get_marker(&input, 4));
    println!("Part 2: {}", get_marker(&input, 14));
}

fn get_marker(input: &str, len: usize) -> usize {
    let mut cur_byte = len;
    let windows = input.as_bytes().windows(len);

    for w in windows {
        if !has_dup(w) {
            return cur_byte;
        }
        cur_byte += 1;
    }
    unreachable!()
}

fn has_dup(slice: &[u8]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}
