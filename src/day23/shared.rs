use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct Point {
    pub(super) x: isize,
    pub(super) y: isize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Point;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<Point> for (isize, isize) {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.0 + rhs.x,
            y: self.1 + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.x;
    }
}

impl AddAssign<(isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<(isize, isize)> for Point {
    fn sub_assign(&mut self, rhs: (isize, isize)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

#[derive(Debug)]
pub(super) enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub(super) fn get(rotate: usize, idx: usize) -> Direction {
        match (rotate + idx) % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => unreachable!(),
        }
    }
}
