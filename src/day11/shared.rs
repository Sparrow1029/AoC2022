use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, newline, one_of, space1},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Mul(Operand, Operand),
    Add(Operand, Operand),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Old,
    Constant(u64),
}

impl Operand {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Constant(c) => c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub num_items_inspected: u64,
    pub operation: Operation,
    pub divisor: u64,
    pub receivers: (usize, usize),
}

pub fn parse_monkey(txt: &str) -> IResult<&str, Monkey> {
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    let (txt, _) = tuple((tag("Monkey "), cc::u64, tag(":"), newline))(txt)?;
    let (txt, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cc::u64),
        newline,
    ))(txt)?;
    let (txt, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, newline))(txt)?;
    let (txt, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cc::u64, newline))(txt)?;
    let (txt, (_, _, rcv1, _)) =
        tuple((space1, tag("If true: throw to monkey "), cc::u64, newline))(txt)?;
    let (txt, (_, _, rcv2)) = tuple((space1, tag("If false: throw to monkey "), cc::u64))(txt)?;

    let vec_items = VecDeque::from(items);
    Ok((
        txt,
        Monkey {
            num_items_inspected: 0,
            items: vec_items,
            operation,
            divisor,
            receivers: (rcv1 as usize, rcv2 as usize),
        },
    ))
}

fn parse_operation(txt: &str) -> IResult<&str, Operation> {
    let (txt, (left, op, right)) = preceded(
        tag("new = "),
        tuple((
            parse_operand,
            preceded(space1, one_of("*+")),
            preceded(space1, parse_operand),
        )),
    )(txt)?;
    let op = match op {
        '*' => Operation::Mul(left, right),
        '+' => Operation::Add(left, right),
        _ => unreachable!(),
    };
    Ok((txt, op))
}

fn parse_operand(txt: &str) -> IResult<&str, Operand> {
    alt((
        value(Operand::Old, tag("old")),
        map(cc::u64, Operand::Constant),
    ))(txt)
}
