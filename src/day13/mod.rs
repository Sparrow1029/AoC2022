mod parse;

pub fn run() {
    println!("\n=== Day 13 ===");
    let input = include_str!("sample_input.txt");
    for pair_str in input.split("\n\n") {
        // println!("{pair_str:#?}");
        let (_v1, _v2) = parse::parse_pair(pair_str).unwrap();
        // println!("{v1:?}, {v2:?}");
    }
    println!("Part 1: ");
    println!("Part 2: ");
}
