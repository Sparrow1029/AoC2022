use super::shared::Point;
use super::HALFWAY;
use itertools::Itertools;
use std::collections::HashSet;

use nom::{bytes::complete::tag, character::complete as cc, sequence::tuple, IResult};

pub fn parse_pair(p: &str) -> IResult<&str, (usize, usize)> {
    let (s, (l, _, r)) = tuple((cc::u64, tag(","), cc::u64))(p)?;
    Ok((s, (normalize(l), r as usize)))
}

fn normalize(x: u64) -> usize {
    let diff = (x as isize - 500) + HALFWAY as isize;
    diff as usize
}

pub fn parse_line(l: &str) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let pairs = l.split(" -> ").into_iter().tuple_windows();
    for (a, b) in pairs {
        let (_, p1) = parse_pair(a).unwrap_or_else(|e| panic!("error parsing digit pair {e}"));
        let (_, p2) = parse_pair(b).unwrap_or_else(|e| panic!("error parsing digit pair {e}"));
        walls.extend(get_wall(p1, p2))
    }
    Ok(walls.into_iter().collect())
}

pub(super) fn get_wall(a: Point, b: Point) -> Vec<Point> {
    let mut points = vec![a, b];
    points.sort_by(|a, b| a.0.cmp(&b.0));
    points.sort_by(|a, b| a.1.cmp(&b.1));
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];

    points.clear();
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            points.push((x, y));
        }
    }
    points
}
