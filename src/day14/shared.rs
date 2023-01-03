use super::parse::parse_line;
use super::{Part, HEIGHT, WIDTH};
use crate::shared::GridError;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

const DEBUG: bool = false;
const DURATION: Duration = Duration::from_millis(500);

pub type Point = (usize, usize);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Tile {
    #[default]
    Empty,
    Wall,
    Sand,
}

#[derive(Clone)]
pub(super) struct Cave {
    pub(super) data: [Tile; WIDTH * HEIGHT],
    pub(super) floor_y: usize,
}

impl Cave {
    pub(super) fn new() -> Self {
        Self {
            data: [Tile::Empty; WIDTH * HEIGHT],
            floor_y: 0,
        }
    }

    pub(super) fn in_bounds(&self, p: Point) -> bool {
        p.0 < WIDTH && p.1 < HEIGHT
    }

    pub(super) fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.data[y * WIDTH + x]
    }

    pub(super) fn set_tile(&mut self, x: usize, y: usize, val: Tile) {
        self.data[y * WIDTH + x] = val;
    }

    fn get_max_y(data: &[Tile; WIDTH * HEIGHT]) -> Result<usize, GridError> {
        for (i, row) in data.array_chunks::<WIDTH>().rev().enumerate() {
            if row.iter().any(|t| *t == Tile::Wall) {
                return Ok((HEIGHT - i) + 1);
            }
        }
        Err(GridError::NotFound)
    }

    pub(super) fn create_floor(&mut self) {
        for x in 0..WIDTH {
            self.set_tile(x, self.floor_y, Tile::Wall);
        }
    }

    pub(super) fn count(&self, kind: Tile) -> usize {
        self.data.iter().filter(|t| *t == &kind).count()
    }

    fn fall_down(&self, x: usize, y: usize, part: Part) -> Option<Point> {
        let mut next_y = y;
        loop {
            next_y += 1;
            if !self.in_bounds((x, next_y)) {
                return None;
            }
            if next_y == self.floor_y {
                match part {
                    Part::One => return None,
                    Part::Two => return Some((x, next_y - 1)),
                }
            }
            if self.get_tile(x, next_y) != Tile::Empty {
                break;
            }
        }
        Some((x, next_y - 1))
    }

    fn get_resting_spot(&mut self, x: usize, y: usize, part: Part) -> bool {
        let mut next_x = x;
        let mut next_y = y;
        'outer: loop {
            if let Some((x2, y2)) = self.fall_down(next_x, next_y, part) {
                let l = (x2 - 1, y2 + 1);
                let r = (x2 + 1, y2 + 1);

                if self.get_tile(l.0, l.1) == Tile::Empty {
                    (next_x, next_y) = l;
                    continue 'outer;
                }
                if self.get_tile(r.0, r.1) == Tile::Empty {
                    (next_x, next_y) = r;
                    continue 'outer;
                }
                if part == Part::Two && (x2, y2) == (x, y) {
                    self.set_tile(x, y, Tile::Sand);
                    return false;
                }
                self.set_tile(x2, y2, Tile::Sand);
                if DEBUG {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("\n{self:?}\n\n");
                    sleep(DURATION);
                }
                break 'outer true;
            }
            return false;
        }
    }

    pub(super) fn pour_sand(&mut self, start_x: usize, start_y: usize, part: Part) {
        if part == Part::Two {
            self.create_floor();
        }
        loop {
            if !self.get_resting_spot(start_x, start_y, part) {
                break;
            }
        }
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.get_tile(x, y) {
                    Tile::Wall => f.write_str("#")?,
                    Tile::Empty => f.write_str(".")?,
                    Tile::Sand => f.write_str("o")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut cave = Cave::new();
        input
            .trim()
            .lines()
            .flat_map(|l| parse_line(l).expect("error parsing line"))
            .for_each(|(x, y)| cave.set_tile(x, y, Tile::Wall));

        cave.floor_y = Cave::get_max_y(&cave.data).unwrap();
        cave
    }
}
