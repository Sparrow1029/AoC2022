pub(super) mod parse;
pub(super) mod shared;

use shared::Tile;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 163;
pub const HALFWAY: usize = WIDTH / 2;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Part {
    One,
    Two,
}

pub fn run() {
    println!("\n=== Day 14 ===");
    let input = include_str!("input.txt");

    let mut cave = shared::Cave::from(input);
    let mut cave2 = cave.clone();

    cave.pour_sand(HALFWAY, 0, Part::One);
    // println!("Cave 1 final:\n{cave:?}");
    println!("Part 1: {}", cave.count(Tile::Sand));

    cave2.pour_sand(HALFWAY, 0, Part::Two);
    // println!("Cave 2 final:\n{cave2:?}");
    println!("Part 2: {}", cave2.count(Tile::Sand));
}
