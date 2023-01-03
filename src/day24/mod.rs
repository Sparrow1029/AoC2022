mod common;
use common::*;
use crossterm::{
    self, execute,
    terminal::{Clear, ClearType},
};
use std::{collections::HashSet, io::stdout};

const DEBUG: bool = false;

fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), Clear(ClearType::All))
}

fn bfs(state: &mut MapState) {
    let start = state.start;
    let goal = state.goal;

    let mut frontier = HashSet::new();
    frontier.insert(start);

    while !frontier.contains(&goal) {
        state.move_blizzards();
        if DEBUG {
            _ = state.display();
        }
        frontier = HashSet::from_iter(explore_frontier(state, &frontier));
    }
}

fn explore_frontier(cur_state: &MapState, frontier: &HashSet<Point>) -> Vec<Point> {
    let mut valid = vec![];
    for loc in frontier {
        for neighbor in loc.adjacent() {
            if cur_state.is_valid(neighbor) {
                valid.push(neighbor);
            }
        }
        if cur_state.is_valid(*loc) {
            valid.push(*loc);
        }
    }
    valid
}

#[allow(unused_must_use)]
pub fn run() {
    println!("\n=== Day 24 ===");
    let input = include_str!("input.txt");
    let mut state = MapState::from(input);
    let mut leg_times = vec![];

    if DEBUG {
        clear_screen();
    }

    bfs(&mut state);
    leg_times.push(state.time);
    println!("Part 1: {}", state.time);

    // Part two, go back to start & then back to end again bc
    // elves forgot their snacks.

    // Go back to start
    std::mem::swap(&mut state.start, &mut state.goal);
    bfs(&mut state);
    leg_times.push(state.time - leg_times.iter().sum::<usize>());

    // Go back to end
    std::mem::swap(&mut state.start, &mut state.goal);
    bfs(&mut state);
    leg_times.push(state.time - leg_times.iter().sum::<usize>());
    if DEBUG {
        println!("Leg times: {leg_times:?}");
    }
    println!("Part 2: {}", state.time);
}
