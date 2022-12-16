use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);
        alt((value(Self::Noop, noop), map(addx, Self::Addx)))(i)
    }

    fn cycles(self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

pub fn run() {}
