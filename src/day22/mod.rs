mod shared;

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{alpha1, digit1},
    IResult,
};
use shared::{Coord, Direction};

type EdgePairs = HashMap<Coord, Coord>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Void,
    Wall,
    Floor,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Turn(Direction),
    Move(isize),
}

#[derive(Clone, Debug)]
struct Player {
    pos: Coord,
    facing: Direction,
    visited: HashSet<(Direction, Coord)>,
}

impl Player {
    fn new(start: Coord) -> Self {
        Self {
            pos: start,
            facing: Direction::Right,
            visited: HashSet::from_iter(vec![(Direction::Right, start)]),
        }
    }

    #[cfg(debug_assertions)]
    fn get_symbol(&self, coord: Coord) -> Option<char> {
        let visited = self
            .visited
            .iter()
            .filter(|(_, c)| *c == coord)
            .map(|(d, _)| *d)
            .collect::<Vec<Direction>>();
        if let Some(dir) = visited.iter().next() {
            match dir {
                Direction::Up => {
                    return Some('^');
                }
                Direction::Down => {
                    return Some('v');
                }
                Direction::Left => {
                    return Some('<');
                }
                Direction::Right => {
                    return Some('>');
                }
            }
        }
        None
    }

    fn execute(&mut self, instruction: Instruction, map: &Map) {
        match instruction {
            Instruction::Move(dist) => {
                for _ in 0..dist {
                    let mut next_pos = match self.facing {
                        Direction::Right => self.pos + (1, 0).into(),
                        Direction::Left => self.pos + (-1, 0).into(),
                        Direction::Up => self.pos + (0, -1).into(),
                        Direction::Down => self.pos + (0, 1).into(),
                    };
                    if !map.in_bounds(next_pos) || map.get_tile(next_pos) == TileType::Void {
                        next_pos = match self.facing {
                            Direction::Left | Direction::Right => map.lr_edges[&self.pos],
                            Direction::Up | Direction::Down => map.ud_edges[&self.pos],
                        };
                    }
                    match map.get_tile(next_pos) {
                        TileType::Floor => {
                            self.pos = next_pos;
                            self.visited.insert((self.facing, self.pos));
                        }
                        TileType::Wall => break,
                        _ => unreachable!(),
                    }
                }
            }
            Instruction::Turn(dir) => {
                self.facing = self.facing.turn(dir);
                self.visited.insert((self.facing, self.pos));
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<TileType>,
    width: isize,
    height: isize,
    start: Coord,
    ud_edges: EdgePairs,
    lr_edges: EdgePairs,
}

impl Map {
    fn get_tile(&self, coord: Coord) -> TileType {
        assert!(self.in_bounds(coord));
        self.tiles[coord.y as usize * self.width as usize + coord.x as usize]
    }

    #[allow(unused)]
    #[cfg(debug_assertions)]
    fn display(&self, player: &Player) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = (x, y).into();
                if let Some(c) = player.get_symbol(coord) {
                    if player.pos == coord {
                        print!("X");
                    } else {
                        print!("{c}");
                    }
                    continue;
                }
                match self.get_tile(coord) {
                    TileType::Void => print!(" "),
                    TileType::Wall => print!("#"),
                    TileType::Floor => print!("."),
                }
            }
            println!();
        }
    }

    #[allow(unused)]
    #[cfg(debug_assertions)]
    fn display_edges(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let key = (x, y).into();
                if self.lr_edges.contains_key(&key) || self.ud_edges.contains_key(&key) {
                    print!("X")
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn in_bounds(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.x >= 0 && coord.y < self.height && coord.y >= 0
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let width = input
            .lines()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len() as isize;
        let height = input.lines().count() as isize;

        let mut tiles = vec![];
        for line in input.lines() {
            let chars = line.chars().collect_vec();
            for x in 0..width as usize {
                if x >= chars.len() {
                    tiles.push(TileType::Void)
                } else {
                    match chars[x] {
                        '.' => tiles.push(TileType::Floor),
                        '#' => tiles.push(TileType::Wall),
                        ' ' => tiles.push(TileType::Void),
                        _ => unreachable!(),
                    }
                }
            }
        }
        let start = (
            tiles.iter().position(|t| *t == TileType::Floor).unwrap() as isize,
            0_isize,
        )
            .into();
        // find L<->R edges
        let mut lr_edges = HashMap::new();
        for y in 0..height {
            let start_x = (0..width)
                .position(|x| tiles[(y * width + x) as usize] != TileType::Void)
                .unwrap() as isize;
            let end_x = width
                - ((0..width)
                    .rev()
                    .position(|x| tiles[(y * width + x) as usize] != TileType::Void)
                    .unwrap() as isize
                    + 1);
            lr_edges.insert((start_x, y).into(), (end_x, y).into());
            lr_edges.insert((end_x, y).into(), (start_x, y).into());
        }
        // find Up/down edges
        let mut ud_edges = HashMap::new();
        for x in 0..width {
            let start_y = (0..height)
                .position(|y| tiles[(y * width + x) as usize] != TileType::Void)
                .unwrap() as isize;
            let end_y = height
                - ((0..height)
                    .rev()
                    .position(|y| tiles[(y * width + x) as usize] != TileType::Void)
                    .unwrap() as isize
                    + 1);
            ud_edges.insert((x, start_y).into(), (x, end_y).into());
            ud_edges.insert((x, end_y).into(), (x, start_y).into());
        }
        Self {
            width,
            height,
            tiles,
            start,
            lr_edges,
            ud_edges,
        }
    }
}

fn parse_instr(input: &str) -> IResult<&str, Instruction> {
    let (rem, instr_str) = alt((digit1, alpha1))(input)?;
    let instruction = if instr_str.chars().all(|c| c.is_ascii_digit()) {
        Instruction::Move(instr_str.parse::<isize>().unwrap())
    } else {
        match instr_str {
            "L" => Instruction::Turn(Direction::Left),
            "R" => Instruction::Turn(Direction::Right),
            _ => unreachable!(),
        }
    };
    Ok((rem, instruction))
}

fn parse(input: &str) -> (Map, VecDeque<Instruction>) {
    let split = input.split("\n\n").collect_vec();
    let graph = split[0];
    let mut instr_input = split[1].trim();
    let mut instructions = VecDeque::new();
    while !instr_input.is_empty() {
        let (rem, instr) = parse_instr(instr_input).unwrap();
        instr_input = rem;
        instructions.push_back(instr);
    }
    (Map::from(graph), instructions)
}

fn part_1(input: &str) -> usize {
    let (map, mut instructions) = parse(input);
    let mut player = Player::new(map.start);
    while let Some(instruction) = instructions.pop_front() {
        player.execute(instruction, &map);
    }
    // map.display(&player);
    (1000 * (player.pos.y + 1) as usize)
        + (4 * (player.pos.x + 1) as usize)
        + player.facing.get_val()
}

pub fn run() {
    println!("\n=== Day 22 ===");
    let input = include_str!("input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: TODO!!");
}
