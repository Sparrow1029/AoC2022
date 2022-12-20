use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

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

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
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

    pub(crate) fn get_neighbors(&self, coord: GridCoord, diag: bool) -> Option<Vec<&T>> {
        if !self.in_bounds(coord) {
            return None;
        }
        let mut neighbors = vec![];
        let mut direction_diffs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        if diag {
            direction_diffs.extend_from_slice(&[(-1, 1), (1, 1), (1, -1), (-1, -1)]);
        }
        for (x, y) in direction_diffs {
            let dx = coord.x as isize + x;
            let dy = coord.y as isize + y;
            if dx < 0 || dx > self.width as isize {
                continue;
            } else if dy < 0 || dy > self.height as isize {
                continue;
            } else {
                neighbors.push(self.cell(coord).unwrap());
            }
        }
        Some(neighbors)
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
