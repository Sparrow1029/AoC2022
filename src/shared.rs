use custom_error::custom_error;
use std::thread::sleep;
use std::time::Duration;
use std::{
    fs::File,
    io::{self, BufRead},
    ops::{Add, AddAssign, Sub, SubAssign},
    path::Path,
};

custom_error! {pub GridError
    OutOfBounds = "coord was out of bounds",
    NotFound = "Not found"
}

pub fn sleep_s(secs: u64, millis: u64) {
    let mut duration = Duration::from_secs(secs);
    duration += Duration::from_millis(millis);
    sleep(duration);
}

/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
/// Get index of max value of collection of items that are Copy
pub fn get_max_index_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridCoord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Add for GridCoord {
    type Output = GridCoord;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Point> for GridCoord {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
pub(crate) struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn in_bounds<C>(&self, coord: C) -> bool
    where
        C: Into<GridCoord>,
    {
        let c = coord.into();
        c.x < self.width && c.y < self.height
    }

    pub(crate) fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    pub(crate) fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn coords(&self) -> Vec<GridCoord> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, _)| (i % self.width, i / self.height).into())
            .collect()
    }

    pub(crate) fn get_neighbors<C>(&self, coord: C, diag: bool) -> Result<Vec<GridCoord>, GridError>
    where
        C: Into<GridCoord>,
    {
        let mut neighbors = vec![];
        let coord = coord.into();
        assert!(self.in_bounds(coord));
        let mut direction_diffs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        if diag {
            direction_diffs.extend_from_slice(&[(-1, 1), (1, 1), (1, -1), (-1, -1)]);
        }
        for (x, y) in direction_diffs {
            let dx = coord.x as isize + x;
            let dy = coord.y as isize + y;
            if dx < 0 || dx > (self.width as isize) - 1 || dy < 0 || dy > (self.height as isize) - 1
            {
                continue;
            } else {
                neighbors.push((dx as usize, dy as usize).into());
            }
        }
        Ok(neighbors)
    }

    pub(crate) fn get_row(&self, row: usize) -> &[T] {
        let coord = row * self.width;
        &self.data[coord..coord + self.width]
    }

    pub(crate) fn get_col(&self, col: usize) -> Vec<T> {
        assert!(col < self.width);
        let mut column = vec![];
        for i in self.data[col..self.data.len()].iter().step_by(self.width) {
            column.push(i.clone());
        }
        column
    }

    pub(crate) fn rows(&self) -> Vec<&[T]> {
        let mut rows = vec![];
        for i in 0..self.height {
            rows.push(self.get_row(i))
        }
        rows
    }

    pub(crate) fn cols(&self) -> Vec<Vec<T>> {
        let mut cols = vec![];
        for i in 0..self.width {
            cols.push(self.get_col(i));
        }
        cols
    }

    pub(crate) fn iter_rows_with_coord(&self) -> Vec<Vec<((usize, usize), T)>> {
        let mut rows = vec![];
        for (y, row_data) in self.rows().iter().enumerate() {
            let mut row = vec![];
            for (i, v) in row_data.iter().enumerate() {
                row.push(((i, y), v.clone()));
            }
            rows.push(row);
        }
        rows
    }

    pub(crate) fn iter_cols_with_coord(&self) -> Vec<Vec<((usize, usize), T)>> {
        let mut cols = vec![];
        for (x, col_data) in self.cols().iter().enumerate() {
            let mut col = vec![];
            for (i, v) in col_data.iter().enumerate() {
                col.push(((x, i), v.clone()));
            }
            cols.push(col);
        }
        cols
    }
}

pub(crate) fn get_neighbors<C>(coord: C, w: usize, h: usize, diag: bool) -> Option<Vec<Point>>
where
    C: Into<Point>,
{
    let mut neighbors = vec![];
    let coord = coord.into();

    let mut direction_diffs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    if diag {
        direction_diffs.extend_from_slice(&[(-1, 1), (1, 1), (1, -1), (-1, -1)]);
    }
    for (x, y) in direction_diffs {
        let dx = coord.x as isize + x;
        let dy = coord.y as isize + y;
        if dx < 0 || dx > (w as isize) - 1 || dy < 0 || dy > (h as isize) - 1 {
            continue;
        } else {
            neighbors.push((dx as usize, dy as usize).into());
        }
    }
    Some(neighbors)
}
