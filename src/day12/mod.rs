use std::fmt::Debug;

use crate::shared::{Grid, GridCoord, GridError};

trait BasicDerive: Default + Debug + Clone + Copy {}

trait Height {
    fn height(self) -> u8;
}

#[derive(Debug, Clone, Copy, Default)]
enum Cell {
    #[default]
    Start,
    End,
    Tile(u8),
}

impl BasicDerive for Cell {}

impl Height for Cell {
    fn height(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Tile(c) => c,
        }
    }
}

impl Grid<Cell> {
    fn walkable_neighbors(&self, coord: GridCoord) -> Result<Vec<GridCoord>, GridError> {
        let curr_height = self.cell(coord).unwrap().height();
        let nbrs = self.get_neighbors(coord, false)?;
        Ok(nbrs
            .iter()
            .filter(move |c| {
                let other_height = self.cell(**c).unwrap().height();
                other_height <= curr_height + 1
            })
            .cloned()
            .collect())
    }

    fn walk(&mut self) {}
}

fn cell_grid_from_input(input: &str) -> Grid<Cell> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let data = input
        .chars()
        .filter(|c| !['\n', '\r'].contains(c))
        .map(|c| match c {
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => Cell::Tile(c as u8 - b'a'),
        })
        .collect::<Vec<Cell>>();
    Grid {
        width,
        height,
        data,
    }
}

pub fn run() {
    println!("\n=== Day 12 ===");
    let input = include_str!("sample_input.txt");
    println!("INPUT: {input}");
    let grid = cell_grid_from_input(input);
    println!("{grid:?}");
    println!("{:?}", grid.walkable_neighbors((2, 3).into()).unwrap());
}
