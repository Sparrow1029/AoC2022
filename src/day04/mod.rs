use crate::shared::read_lines;

#[derive(Debug)]
struct SectionAssignmentPair {
    p1: (usize, usize),
    p2: (usize, usize),
}

impl SectionAssignmentPair {
    fn new(input_string: &String) -> Self {
        let values = input_string
            .split(|c| c == ',' || c == '-')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        SectionAssignmentPair {
            p1: (values[0], values[1]),
            p2: (values[2], values[3]),
        }
    }

    fn has_fully_contained_assignment(&self) -> bool {
        ((self.p1.0 <= self.p2.0) && (self.p1.1 >= self.p2.1))
            || ((self.p2.0 <= self.p1.0) && (self.p2.1 >= self.p1.1))
    }

    fn overlaps(&self) -> bool {
        ((self.p1.0 >= self.p2.0) && (self.p1.0 <= self.p2.1))
            || ((self.p2.0 >= self.p1.0) && (self.p2.0 <= self.p1.1))
    }
}

fn part1(input: &Vec<String>) {
    let mut total = 0;
    for line in input {
        let pairs = SectionAssignmentPair::new(line);
        if pairs.has_fully_contained_assignment() {
            total += 1;
        }
    }
    println!("Part 1: {total}");
}

fn part2(input: &Vec<String>) {
    let mut total = 0;
    for line in input {
        let pairs = SectionAssignmentPair::new(line);
        if pairs.overlaps() {
            total += 1;
        }
    }
    println!("Part 2: {total}");
}

pub fn run() {
    println!("\n=== Day 04 ===");
    let input: Vec<String> = read_lines("src/day04/input.txt")
        .expect("error reading file")
        .map(|line| line.expect("error reading line"))
        .collect();
    part1(&input);
    part2(&input);
}
