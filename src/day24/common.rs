/// Blizzards! AoC 2022 Day 24
///
/// Various entities represented with flags/as bits of u8 integer:
/// ```ignore
/// 0000 0000
///   || ↑↓←→ Blizzard (Direction)
///   |└----- Wall
///   └------ Expedition
/// ```
/// 0 represents an empty tile.
use crossterm::{
    cursor, execute, queue,
    style::{Attribute, Color, Print, StyledContent, Stylize},
    Result as CtermResult,
};

use crate::shared::sleep_s;
use std::{io::stdout, isize, mem::swap};

pub(super) const WALL: u8 = 0b0001_0000;
// pub(super) const EXPEDITION: u8 = 0b0010_0000;
pub(super) const UP: u8 = 0b0000_1000;
pub(super) const DOWN: u8 = 0b0000_0100;
pub(super) const LEFT: u8 = 0b0000_0010;
pub(super) const RIGHT: u8 = 0b0000_0001;

const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add<(isize, isize)> for Point {
    type Output = Point;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Point {
            x: (self.x as isize + rhs.0) as usize,
            y: (self.y as isize + rhs.1) as usize,
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn adjacent(&self) -> Vec<Point> {
        let mut adjacent = vec![];
        for pt in VECTORS {
            let new_x = pt.0 + self.x as isize;
            let new_y = pt.1 + self.y as isize;
            if new_x < 0 || new_y < 0 {
                continue;
            }
            adjacent.push((new_x as usize, new_y as usize).into())
        }
        adjacent
    }
}

/// Makes it easier to match on a coord to display tiles for visual debug
pub enum TileType {
    Empty,
    Wall,
    Blizzard(u8),
    Expedition,
}

impl TileType {
    pub fn get_symbol(&self) -> StyledContent<&str> {
        match self {
            TileType::Wall => "▢".with(Color::Green),
            TileType::Empty => "·".with(Color::DarkGrey),
            TileType::Blizzard(dir) => match dir.count_ones() {
                1 => {
                    if dir & UP != 0 {
                        "▲".with(Color::Blue)
                    } else if dir & DOWN != 0 {
                        "▼".with(Color::Blue)
                    } else if dir & LEFT != 0 {
                        "◄".with(Color::Blue)
                    } else if dir & RIGHT != 0 {
                        "►".with(Color::Blue)
                    } else {
                        panic!("Unhandled u8: {dir:<08b}");
                    }
                }
                2 => "2".with(Color::Blue).attribute(Attribute::Bold),
                3 => "3".with(Color::Blue).attribute(Attribute::Bold),
                4 => "4".with(Color::Blue).attribute(Attribute::Bold),
                _ => unreachable!(),
            },
            TileType::Expedition => "E".with(Color::Red).attribute(Attribute::Bold),
        }
    }
}

pub fn get_type(tile: u8) -> TileType {
    match tile {
        0 => TileType::Empty,
        16 => TileType::Wall,
        32 => TileType::Expedition,
        1..=15 => TileType::Blizzard(tile),
        _ => unreachable!(),
    }
}

/// `MapState` holds the current state in a Vec<u8>,
/// calculates the next state into another Vec<u8>,
/// then swaps the pointers to each.
#[derive(Clone)]
pub struct MapState {
    pub width: usize,
    pub height: usize,

    pub start: Point,
    pub goal: Point,

    pub state: Vec<u8>,
    pub next: Vec<u8>,

    pub time: usize,
}

impl std::fmt::Debug for MapState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:<08b} ", self.get_cur_tile((x, y).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MapState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            start: Point::new(1, 0),
            goal: Point::new(width - 2, height - 1),
            state: vec![0; width * height],
            next: vec![0; width * height],
            time: 0,
        }
    }

    pub fn set_next_tile(&mut self, pos: Point, val: u8) {
        self.next[pos.y * self.width + pos.x] |= val;
    }

    pub fn set_cur_tile(&mut self, pos: Point, val: u8) {
        self.state[pos.y * self.width + pos.x] |= val;
    }

    pub fn get_cur_tile(&self, pos: Point) -> u8 {
        self.state[pos.y * self.width + pos.x]
    }

    pub fn move_blizzards(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cur_pos = (x, y).into();
                match get_type(self.get_cur_tile(cur_pos)) {
                    TileType::Wall => self.set_next_tile(cur_pos, WALL),
                    TileType::Empty => {}
                    TileType::Expedition => {}
                    TileType::Blizzard(blizzard) => self.set_next_blizzards(cur_pos, blizzard),
                }
            }
        }
        swap(&mut self.next, &mut self.state);
        self.clear_next();
        self.time += 1;
    }

    fn set_next_blizzards(&mut self, cur_pos: Point, blizzard: u8) {
        if blizzard & UP != 0 {
            let mut mov = cur_pos + (0, -1);
            if mov.y == 0 {
                mov.y = self.height - 2;
            };
            self.set_next_tile(mov, UP);
        }
        if blizzard & DOWN != 0 {
            let mut mov = cur_pos + (0, 1);
            if mov.y == self.height - 1 {
                mov.y = 1;
            };
            self.set_next_tile(mov, DOWN);
        }
        if blizzard & LEFT != 0 {
            let mut mov = cur_pos + (-1, 0);
            if mov.x == 0 {
                mov.x = self.width - 2;
            };
            self.set_next_tile(mov, LEFT);
        }
        if blizzard & RIGHT != 0 {
            let mut mov = cur_pos + (1, 0);
            if mov.x == self.width - 1 {
                mov.x = 1;
            };
            self.set_next_tile(mov, RIGHT);
        }
    }

    fn clear_next(&mut self) {
        self.next.iter_mut().for_each(|t| *t = 0);
    }

    fn in_bounds(&self, pt: (isize, isize)) -> bool {
        if pt.0 < 0 || pt.0 > self.width as isize - 1 || pt.1 < 0 || pt.1 > self.height as isize - 1
        {
            return false;
        }
        true
    }

    pub fn is_valid(&self, pos: Point) -> bool {
        self.in_bounds((pos.x as isize, pos.y as isize)) && self.get_cur_tile(pos) == 0
    }

    pub fn display(&self) -> CtermResult<()> {
        let mut stdout = stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    Print(get_type(self.get_cur_tile((x, y).into())).get_symbol())
                )?;
            }
        }
        execute!(stdout, cursor::MoveTo(0, self.height as u16))?;
        println!("Step: {}", self.time);
        sleep_s(0, 500);
        Ok(())
    }
}

impl From<&str> for MapState {
    fn from(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut mtn = MapState::new(width, height);

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                let pt = (x, y).into();
                match c {
                    '#' => mtn.set_cur_tile(pt, WALL),
                    '^' => mtn.set_cur_tile(pt, 0b0000_1000),
                    'v' => mtn.set_cur_tile(pt, 0b0000_0100),
                    '<' => mtn.set_cur_tile(pt, 0b0000_0010),
                    '>' => mtn.set_cur_tile(pt, 0b0000_0001),
                    '.' => {}
                    _ => unreachable!(),
                }
            })
        });
        // mtn.set_cur_tile((1, 0).into(), EXPEDITION);
        mtn
    }
}
