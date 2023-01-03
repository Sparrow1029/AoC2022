use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal, Result as CtermResult,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    cmp::PartialEq,
    io::stdout,
    mem::discriminant,
    ops::{Add, AddAssign, Sub, SubAssign},
};

// use crate::shared::sleep_s;

const START: Point = Point { x: 1, y: 1 };
pub const INPUT: &str = include_str!("sample_input.txt");
lazy_static! {
    pub static ref WIDTH: usize = INPUT.lines().next().unwrap().len();
    pub static ref HEIGHT: usize = INPUT.lines().count();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn mov(self, pt: Point) -> Point {
        match self {
            Direction::Left => pt - (1, 0).into(),
            Direction::Up => pt + (0, 1).into(),
            Direction::Right => pt + (1, 0).into(),
            Direction::Down => pt - (0, 1).into(),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

pub fn step(mtn: &mut Mountain) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

#[derive(Debug)]
pub struct Tile {
    pub kind: TileType,
    pub pos: Point,
}

impl Tile {
    pub fn new(kind: TileType, pos: Point) -> Self {
        Self { kind, pos }
    }

    pub fn update(&mut self, mtn: &Mountain) {
        match self.kind {
            TileType::Empty | TileType::Wall => {}
            TileType::Blizzard(dir) => {
                let mut new_pos = match dir {
                    Direction::Up => self.pos + (0, 1).into(),
                    Direction::Right => self.pos + (1, 0).into(),
                    Direction::Down => self.pos - (0, 1).into(),
                    Direction::Left => self.pos - (1, 0).into(),
                };
                self.pos = Tile::conserve_blizzard_energy(new_pos, dir, mtn);
            }
            TileType::Expedition => self.update_expedition(mtn),
        }
    }

    pub fn conserve_blizzard_energy(pt: Point, dir: Direction, mtn: &Mountain) -> Point {
        if mtn.get_tiles(pt).iter().next().unwrap().kind == TileType::Wall {
            match dir {
                Direction::Down => return (pt.x, 1_usize).into(), // Skip wall
                Direction::Up => return (pt.x, mtn.height - 2_usize).into(),
                Direction::Left => return (mtn.width - 2_usize, pt.y).into(),
                Direction::Right => return (1_usize, pt.y).into(),
            }
        }
        pt
    }

    pub fn update_expedition(&mut self, mtn: &Mountain) {
        for n in get_neighbors(self.pos) {
            let tiles = mtn.get_tiles(self.pos);
            if tiles.len() > 1 { // it's a blizzard
                continue;
            }
            let tile = tiles.into_iter().next().unwrap();
            match tile.kind {
                TileType::Empty => {
                    self.pos = n;
                    mtn.
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Mountain {
    pub start: Point,
    pub end: Point,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Mountain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            start: (1, 0).into(),
            end: (width - 2, height - 1).into(),
            width,
            height,
            tiles: Vec::with_capacity(width * height),
        }
    }

    pub fn in_bounds(&self, pt: Point) -> bool {
        pt.x > 0 && pt.x < self.width && pt.y > 0 && pt.y < self.height
    }

    pub fn get_tile_indices(&self, pt: Point) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.pos == pt)
            .map(|(i, _)| i)
            .collect_vec()
    }

    pub fn get_tiles(&self, pt: Point) -> Vec<&Tile> {
        self.tiles.iter().filter(|t| t.pos == pt).collect_vec()
    }

    // pub fn get_mut_tiles(&mut self, pt: Point) -> Vec<&mut Tile> {
    //     self.tiles.iter_mut().filter(|t| t.pos == pt).collect_vec()
    // }

    // pub fn set_tile(&mut self, pt: Point, value: TileType) {
    //     self.tiles.iter_mut().find(|t| *t.get_cur_pos() == pt)
    // }

    pub fn move_blizzards(&mut self, pt: Point) {}

    pub fn display(&self) -> CtermResult<()> {
        let mut stdout = stdout();

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        for y in 0..self.height {
            for x in 0..self.width {
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    self.get_tile(x, y).get_symbol()
                )?;
            }
        }
        Ok(())
    }
}

impl From<&str> for Mountain {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let mut mtn = Mountain::new(width, height);

        let tiles = value
            .lines()
            .enumerate()
            .flat_map(move |(y, l)| {
                l.chars().enumerate().map(move |(x, c)| match c {
                    '#' => Tile::new(TileType::Wall, (x, y).into()),
                    '^' | '>' | 'v' | '<' => {
                        Tile::new(TileType::Blizzard(Direction::from(c)), (x, y).into())
                    }
                    '.' => Tile::new(TileType::Empty, (x, y).into()),
                    _ => unreachable!(),
                })
            })
            .collect();
        mtn.tiles = tiles;
        let (x, y) = mtn.start.into();
        mtn.set_tile(x, y, TileType::Expedition(mtn.start));
        mtn
    }
}

#[derive(Debug, Clone, Eq)]
pub enum TileType {
    Wall,
    Empty,
    Blizzard(Direction),
    Expedition,
}

impl TileType {
    pub fn get_symbol(&self) -> style::PrintStyledContent<&str> {
        match self {
            TileType::Wall => style::PrintStyledContent("▢".dark_blue()),
            TileType::Empty => style::PrintStyledContent("·".dark_grey()),
            TileType::Blizzard(dir) => match dir {
                Direction::Up => style::PrintStyledContent("▲".blue()),
                Direction::Down => style::PrintStyledContent("▼".blue()),
                Direction::Left => style::PrintStyledContent("◄".blue()),
                Direction::Right => style::PrintStyledContent("►".blue()),
            },
            TileType::Expedition => style::PrintStyledContent("⊛".red()),
        }
    }
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub fn get_neighbors(pt: Point) -> [Point; 4] {
    [
        pt - (1, 0).into(), // Left
        pt + (0, 1).into(), // Up
        pt + (1, 0).into(), // Right
        pt - (0, 1).into(), // Down
    ]
}
