use std::default;

use crate::shared::{Grid, GridCoord};

#[derive(Debug, Clone, Default)]
enum Cell {
    #[default]
    Start,
    End,
    Tile(u8),
}

impl Cell {
    fn height(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Tile(c) => c - b'a',
        }
    }
}

impl Grid<Cell> {
    fn walkable_neighbors(&self, coord: GridCoord) -> Vec<&Cell> {
        let mut neighbors = self.get_neighbors(coord, false).unwrap();
        neighbors
    }
}
pub fn run() {}
