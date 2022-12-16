use crate::shared::read_lines;
use std::{collections::VecDeque, fmt};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map, value},
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Like [`crate::shared::Coord2d`], except that it accepts negative integers
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for GridPos {
    fn add_assign(&mut self, other: GridPos) {
        *self = GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(i)
    }

    pub(crate) fn delta(self) -> GridPos {
        match self {
            Direction::Up => GridPos { x: 0, y: -1 },
            Direction::Down => GridPos { x: 0, y: 1 },
            Direction::Left => GridPos { x: -1, y: 0 },
            Direction::Right => GridPos { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Instruction {
    pub(crate) dir: Direction,
    pub(crate) dist: u32,
}

impl Instruction {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                preceded(space1, nom::character::complete::u32),
            )),
            |(dir, dist)| Self { dir, dist },
        )(i)
    }
}

pub(crate) fn parse_input(path: &str) -> Result<VecDeque<Instruction>, std::io::Error> {
    let lines = read_lines(path)?;
    Ok(lines
        .map(|l| {
            if let Ok(l_val) = l {
                all_consuming(Instruction::parse)(l_val.as_str())
                    .finish()
                    .unwrap()
                    .1
            } else {
                panic!("Couldn't parse instruction from line")
            }
        })
        .collect())
}
