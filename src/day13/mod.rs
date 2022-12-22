mod parse;

pub fn run() {
    let input = include_str!("sample_input.txt");
    for pair_str in input.split("\n\n") {
        // println!("{pair_str:#?}");
        let (v1, v2) = parse::parse_pair(pair_str).unwrap();
        // println!("{v1:?}, {v2:?}");
    }
}
