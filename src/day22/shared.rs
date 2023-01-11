use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct Coord {
    pub(super) x: isize,
    pub(super) y: isize,
}

impl Add<(isize, isize)> for Coord {
    type Output = Coord;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl AddAssign<(isize, isize)> for Coord {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(super) fn turn(self, dir: Direction) -> Direction {
        match self {
            Direction::Up => match dir {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                _ => unreachable!(),
            },
            Direction::Down => match dir {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                _ => unreachable!(),
            },
            Direction::Left => match dir {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                _ => unreachable!(),
            },
            Direction::Right => match dir {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                _ => unreachable!(),
            },
        }
    }

    pub(super) fn get_val(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}
