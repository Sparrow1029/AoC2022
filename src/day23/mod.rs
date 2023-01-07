use crate::shared::sleep_s;
use std::collections::{HashMap, HashSet};
mod shared;
const N: [(isize, isize); 3] = [(0, -1), (1, -1), (-1, -1)]; // Look N, NE, NW
const S: [(isize, isize); 3] = [(0, 1), (1, 1), (-1, 1)]; // Look S, SE, SW
const W: [(isize, isize); 3] = [(-1, 0), (-1, -1), (-1, 1)]; // Look W, NW, SW
const E: [(isize, isize); 3] = [(1, 0), (1, -1), (1, 1)]; // Look E, NE, SE
const DIRECTIONS: [[[(isize, isize); 3]; 4]; 4] =
    [[N, S, W, E], [S, W, E, N], [W, E, N, S], [E, N, S, W]];

#[rustfmt::skip]
use shared::{Direction, Point};

type ElfPosMap = HashMap<usize, Point>;
type PosElfMap = HashMap<Point, usize>;

pub(self) struct Grove {
    pub(self) elf_map: ElfPosMap,
    pub(self) moves: PosElfMap,
    pub(self) proposals: HashSet<Point>,
    pub(self) step: usize,
}

impl Grove {
    fn parse(input: &str) -> ElfPosMap {
        let mut cur_elf_id: usize = 0;
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, c)| match c {
                        '.' => None,
                        '#' => {
                            cur_elf_id += 1;
                            Some((cur_elf_id, Point::from((x as isize, y as isize))))
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<(usize, Point)>>()
            })
            .collect()
    }

    fn propose_move(&self, elf_pos: Point, other_elves_pos: &[Point]) -> Option<Point> {
        // represent all spaces with an elf in them surrounding this elf as false, empty as true
        // f t t
        // f e t
        // f f t
        let idx = self.step % 4;
        let valid_spaces = DIRECTIONS[idx]
            .iter()
            .map(|d| d.map(|pt| !other_elves_pos.contains(&(elf_pos + pt))))
            .collect::<Vec<[bool; 3]>>();
        // if all points around the elf are empty (false), return None
        let all_empty = valid_spaces.flatten().iter().all(|pt| *pt);
        if all_empty {
            None
        } else {
            // otherwise, find the first valid direction
            let first_empty_direction = valid_spaces
                .iter()
                .map(|dir| dir.iter().all(|b| *b))
                .position(|x| x);
            if let Some(dir) = first_empty_direction {
                let pt: Point = match Direction::get(idx, dir) {
                    Direction::North => elf_pos + (0, -1),
                    Direction::South => elf_pos + (0, 1),
                    Direction::East => elf_pos + (1, 0),
                    Direction::West => elf_pos + (-1, 0),
                };
                return Some(pt);
            }
            None
        }
    }

    fn get_proposed_moves(&mut self) -> bool {
        let elf_positions: Vec<Point> = self.elf_map.values().cloned().collect();
        for elf_id in 1..=self.elf_map.len() {
            if let Some(mov) = self.propose_move(self.elf_map[&elf_id], &elf_positions) {
                if !self.proposals.insert(mov) {
                    // value was already seen, remove from moves;
                    self.moves.remove(&mov);
                } else {
                    self.moves.insert(mov, elf_id);
                }
            }
        }
        if self.proposals.is_empty() {
            return false;
        }
        self.proposals.clear();
        true
    }

    fn move_elves(&mut self) -> bool {
        if !self.get_proposed_moves() {
            return false;
        }
        for (new_pos, elf) in self.moves.iter() {
            self.elf_map.entry(*elf).and_modify(|e| *e = *new_pos);
        }
        self.moves.clear();
        self.step += 1;
        true
    }

    #[rustfmt::skip]
    fn get_bounds(&self) -> (Point, Point) {
        let min_x = self.elf_map.values().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let max_x = self.elf_map.values().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let min_y = self.elf_map.values().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        let max_y = self.elf_map.values().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        ((min_x, min_y).into(), (max_x, max_y).into())
    }

    fn get_empty_area(&self) -> isize {
        let (min, max) = self.get_bounds();
        ((max.x + 1 - min.x) * (max.y + 1 - min.y)) - self.elf_map.len() as isize
    }

    fn get_map_string(&self) -> String {
        let mut map_string = String::new();

        let inverted: PosElfMap = self.elf_map.iter().map(|(k, v)| (*v, *k)).collect();
        let (min, max) = self.get_bounds();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                match inverted.get(&Point::from((x, y))) {
                    Some(_) => map_string.push('#'),
                    None => map_string.push('.'),
                }
            }
            map_string.push('\n');
        }
        map_string
    }
}

impl From<&str> for Grove {
    fn from(input: &str) -> Self {
        let elf_map = Grove::parse(input);
        Self {
            moves: HashMap::with_capacity(elf_map.len()),
            elf_map,
            proposals: HashSet::new(),
            step: 0,
        }
    }
}

pub fn run() {
    println!("\n=== Day 23 ===");
    let input = include_str!("input.txt");
    let mut grove = Grove::from(input);
    while grove.move_elves() {
        // print!("\x1B[2J\x1B[1;1H");
        // println!("{}", grove.get_map_string());
        if grove.step == 10 {
            println!("Part 1: {}", grove.get_empty_area());
        }
    }
    println!("Part 2: {}", grove.step + 1)
}
