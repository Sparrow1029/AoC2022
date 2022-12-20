/// Super inefficient way to do Day 8 of AoC 2022 -- but it does work, so yay.
use crate::shared::{Grid, GridCoord};
use std::collections::HashSet;

#[allow(dead_code)]
const SAMPLE_INPUT: &str = "\
30373
25512
65332
33549:wa

35390";

fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }

    grid
}

/// Get total visible trees in the grid
fn get_total_visible_pt1(grid: &Grid<usize>) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::with_capacity(grid.height() * grid.width());
    // iterate through rows, forward & backward
    for row in grid.iter_rows_with_coord() {
        visible.insert(row[0].0);
        let mut cur_max = row[0].1;
        for (coord, val) in row.iter().skip(1) {
            if val > &cur_max {
                cur_max = *val;
                visible.insert(*coord);
            }
        }
        let last = row.last().unwrap();
        visible.insert(last.0);
        cur_max = last.1;
        for (coord, val) in row.iter().rev().skip(1) {
            if val > &cur_max {
                cur_max = *val;
                visible.insert(*coord);
            }
        }
    }

    // iterate through columns, forward & backward
    for col in grid.iter_cols_with_coord() {
        visible.insert(col[0].0);
        let mut cur_max = col[0].1;
        for (coord, val) in col.iter().skip(1) {
            if val > &cur_max {
                cur_max = *val;
                visible.insert(*coord);
            }
        }
        let last = col.last().unwrap();
        visible.insert(last.0);
        cur_max = last.1;
        for (coord, val) in col.iter().rev().skip(1) {
            if val > &cur_max {
                cur_max = *val;
                visible.insert(*coord);
            }
        }
    }

    visible
}

#[allow(dead_code)]
fn dbg_visible(visible: &HashSet<(usize, usize)>, grid: &Grid<usize>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if visible.contains(&(x, y)) {
                print!("@ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn get_scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let (left, right) = grid.get_row(coord.y).split_at(coord.x);
    let col = grid.get_col(coord.x);
    let (up, down) = col.split_at(coord.y);

    let mut scores: [usize; 4] = [0; 4];

    // get score looking left
    let tree_val = grid.cell(coord).unwrap();
    for val in left.iter().rev() {
        scores[0] += 1;
        if val >= tree_val {
            break;
        }
    }

    // get score looking right
    for val in right.iter().skip(1) {
        // skip first val since that's the tree in question
        scores[1] += 1;
        if val >= tree_val {
            break;
        }
    }

    // get score looking up
    for val in up.iter().rev() {
        scores[2] += 1;
        if val >= tree_val {
            break;
        }
    }

    // get score looking down
    for val in down.iter().skip(1) {
        // skip tree itself
        scores[3] += 1;
        if val >= tree_val {
            break;
        }
    }

    scores.iter().product()
}

fn part2(grid: &Grid<usize>) {
    let mut max = 0;
    grid.coords().iter().for_each(|coord| {
        let score = get_scenic_score(grid, *coord);
        if score > max {
            max = score;
        }
    });
    println!("Part 2: {max}");
}

pub fn run() {
    println!("\n=== Day 08 ===");
    let input_str = include_str!("input.txt");
    let grid = parse_grid(input_str);

    let visible = get_total_visible_pt1(&grid);
    println!("Part 1: {}", visible.len());
    part2(&grid);
}
