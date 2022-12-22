use serde::Deserialize;

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{self as cc},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

fn parse_list_json(s: &str) {}

fn closing_brackets(s: &str) -> IResult<&str, &str> {
    let sbracks = s.chars().take_while(|c| *c == ']');
    let (s, rem) = take_while(|c| c == ']' || c == ',')(s)?;
    println!("s: {s}, rem: {rem}\n");
    Ok((s, rem))
}

pub(super) fn parse_list(input: &str) -> IResult<&str, (Vec<u8>, &str)> {
    println!("Parsing: {input}");
    let (s, (_, vec, rem)) = tuple((
        take_while(|c| c == '['),
        separated_list0(tag(","), cc::u8),
        closing_brackets,
    ))(input)?;
    Ok((s, (vec, rem)))
}

pub(super) fn parse_pair(pair_str: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let (str1, mut str2) = pair_str.split_at(pair_str.find('\n').unwrap());
    str2 = &str2[1..]; // skip
    println!("{str1:#?}, {str2:#?}");
    let vec1 = parse_list(str1).unwrap().1;
    let vec2 = parse_list(str2).unwrap().1;
    println!("{vec1:?}, {vec2:?}\n\n");
    Ok((pair_str, (vec![], vec![])))
}
