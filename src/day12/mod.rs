/// Really inefficent way to do Day 12 (I'm sure). Tried to improve by allocating HashMap & VecDequeue only once, but didn't help much.
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

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

    fn get_start_end(&self) -> (GridCoord, GridCoord) {
        // Uninitialized vals won't work here.
        let mut start = GridCoord { x: 0, y: 0 };
        let mut end = GridCoord { x: 0, y: 0 };
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell = self.cell((x, y).into()).unwrap();
                match cell {
                    Cell::Start => start = GridCoord { x, y },
                    Cell::End => end = GridCoord { x, y },
                    _ => {}
                }
            }
        }
        (start, end)
    }

    fn best_first_search(
        &self,
        start: GridCoord,
        end: GridCoord,
        parent: &mut HashMap<GridCoord, Option<GridCoord>>,
        queue: &mut VecDeque<GridCoord>,
    ) -> Option<Vec<GridCoord>> {
        queue.push_back(start);
        parent.insert(start, None);
        while let Some(curr) = queue.pop_front() {
            if !self.in_bounds(curr) {
                continue;
            }
            if curr == end {
                queue.clear();
                break;
            }
            let neighbors = self.walkable_neighbors(curr).unwrap();
            for nbr in neighbors.iter() {
                if !parent.contains_key(nbr) {
                    parent.insert(*nbr, Some(curr));
                    queue.push_back(*nbr);
                }
            }
        }
        backtrace(parent, start, end)
    }

    fn get_coords_of_val(&self, val: u8) -> Vec<GridCoord> {
        let mut coords = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let coord = (x, y).into();
                let cell = self.cell(coord).unwrap();
                if cell.height() == val {
                    coords.push(coord);
                }
            }
        }
        coords
    }
}

fn backtrace(
    parent: &mut HashMap<GridCoord, Option<GridCoord>>,
    start: GridCoord,
    end: GridCoord,
) -> Option<Vec<GridCoord>> {
    if !parent.contains_key(&end) {
        return None;
    }
    let mut path = Vec::new();
    path.push(end);
    while path.last().unwrap() != &start {
        path.push(parent.get(path.last().unwrap()).unwrap().unwrap());
    }
    parent.clear();
    path.reverse();
    Some(path)
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

fn part1(
    grid: &Grid<Cell>,
    parent: &mut HashMap<GridCoord, Option<GridCoord>>,
    queue: &mut VecDeque<GridCoord>,
) -> usize {
    let (start, end) = grid.get_start_end();
    let res = grid.best_first_search(start, end, parent, queue).unwrap();
    res.len() - 1
}

fn part2(
    grid: &Grid<Cell>,
    parent: &mut HashMap<GridCoord, Option<GridCoord>>,
    queue: &mut VecDeque<GridCoord>,
) -> usize {
    let (_, end) = grid.get_start_end();
    let mut min = usize::MAX;
    for start in &grid.get_coords_of_val(0) {
        if let Some(res) = grid.best_first_search(*start, end, parent, queue) {
            let distance = res.len() - 1;
            if distance < min {
                min = distance;
            }
        }
    }
    min
}

pub fn run() {
    println!("\n=== Day 12 ===");
    let input = include_str!("input.txt");
    let grid = cell_grid_from_input(input);
    let mut parent = HashMap::new();
    let mut queue = VecDeque::new();
    println!("Part 1: {}", part1(&grid, &mut parent, &mut queue));
    println!("Part 2: {}", part2(&grid, &mut parent, &mut queue));
}
