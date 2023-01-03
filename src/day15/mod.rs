use std::collections::HashSet;

use nom::{bytes::complete::tag, character::complete as cc, sequence::tuple, IResult};

fn parse_input(input: &str) -> IResult<&str, Vec<Pair>> {
    let mut pairs = vec![];
    for line in input.lines() {
        let (_, (_, x1, _, y1, _, x2, _, y2)) = tuple((
            tag("Sensor at x="),
            cc::i64,
            tag(", y="),
            cc::i64,
            tag(": closest beacon is at x="),
            cc::i64,
            tag(", y="),
            cc::i64,
        ))(line)?;
        pairs.push(Pair::new((x1, y1).into(), (x2, y2).into()));
    }
    Ok((input, pairs))
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pair {
    pub sensor: Point,
    pub beacon: Point,
    pub manhattan_dist: i64,
}

impl Pair {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        Self {
            sensor,
            beacon,
            manhattan_dist: sensor.manhattan_dist(&beacon),
        }
    }

    pub fn get_xrange_withy(&self, y: i64) -> Option<(i64, i64)> {
        let rng = (self.sensor.y - self.manhattan_dist)..(self.sensor.y + self.manhattan_dist);
        if !rng.contains(&y) {
            return None;
        }

        let left_corner = Point::new(self.sensor.x - self.manhattan_dist, self.sensor.y);
        let right_corner = Point::new(self.sensor.x + self.manhattan_dist, self.sensor.y);
        let top_corner = if y < self.sensor.y {
            (self.sensor.x, self.sensor.y - self.manhattan_dist).into()
        } else {
            (self.sensor.x, self.sensor.y + self.manhattan_dist).into()
        };
        // first line
        let m1 = get_slope(top_corner, left_corner);
        let b1 = get_y_intercept(left_corner, m1);
        let left_x = get_x_from_line(y, m1, b1);

        // second_line
        let m2 = get_slope(top_corner, right_corner);
        let b2 = get_y_intercept(right_corner, m2);
        let right_x = get_x_from_line(y, m2, b2);

        Some((left_x, right_x))
    }
}

fn get_slope(pt1: Point, pt2: Point) -> i64 {
    (pt2.y - pt1.y) / (pt2.x - pt1.x)
}

fn get_y_intercept(pt: Point, slope: i64) -> i64 {
    pt.y - slope * pt.x
}

fn get_x_from_line(y: i64, m: i64, b: i64) -> i64 {
    (y - b) / m
}

fn get_empty_row(pairs: &Vec<Pair>, y: i64) -> i64 {
    let mut min_x = i64::MAX;
    let mut max_x = 0;
    for pair in pairs {
        if let Some((min, max)) = pair.get_xrange_withy(y) {
            if min < min_x {
                min_x = min
            };
            if max > max_x {
                max_x = max
            };
        }
    }
    let beacons: HashSet<i64> = pairs
        .iter()
        .flat_map(|p| {
            if p.beacon.y == y && p.beacon.x <= max_x && p.beacon.x >= min_x {
                Some(p.beacon.x)
            } else {
                None
            }
        })
        .collect();
    let mut res = max_x - min_x - beacons.iter().len() as i64;
    // need to include '0' as point
    if min_x < 0 {
        res += 1;
    }
    res
}

pub fn part2(pairs: &Vec<Pair>, limit: i64) -> Option<u64> {
    let mut ranges = Vec::with_capacity(100);
    let mut stack = Vec::with_capacity(100);
    for y in 0..limit {
        for pair in pairs {
            if let Some((mut left_x, mut right_x)) = pair.get_xrange_withy(y) {
                if left_x < 0 {
                    left_x = 0;
                };
                if right_x > limit {
                    right_x = limit;
                };
                ranges.push((left_x, right_x));
            }
        }
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        stack.push(ranges[0]);
        for rng in ranges.iter().skip(1) {
            let last = stack.last().unwrap();
            let stack_len = stack.len();
            if last.0 <= rng.0 && rng.0 <= last.1 {
                stack[stack_len - 1] = (last.0, std::cmp::max(last.1, rng.1));
            } else {
                stack.push(*rng);
            }
        }
        if stack.len() > 1 {
            for window in stack.windows(2) {
                let (a, b) = (window[0], window[1]);
                let overlapping = non_overlapping(a, b);
                if overlapping == 2 {
                    return Some((b.0 - 1) as u64 * 4_000_000 + y as u64);
                }
            }
        }
        ranges.clear();
        stack.clear();
    }
    None
}

pub fn non_overlapping(rng1: (i64, i64), rng2: (i64, i64)) -> i64 {
    if rng2.0 < rng1.1 {
        return 0;
    }
    rng2.0 - rng1.1
}

pub fn run() {
    println!("\n=== Day 15 ===");
    let input = include_str!("input.txt");
    let pairs = parse_input(input).unwrap().1;

    println!("Part 1: {}", get_empty_row(&pairs, 2_000_000));

    let tuning_frequency = part2(&pairs, 4_000_000).unwrap();
    println!("Part 2: {tuning_frequency:?}");
}
