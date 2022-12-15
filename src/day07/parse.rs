use crate::shared::read_lines;
use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::io::Error;

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

#[derive(Debug)]
struct Ls;

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

pub(super) fn parse_input(path: &str) -> Result<Vec<Line>, Error> {
    let lines = read_lines(path)?;
    Ok(lines
        .map(|l| {
            if let Ok(l_val) = l {
                all_consuming(parse_line)(l_val.as_str())
                    .finish()
                    .unwrap()
                    .1
            } else {
                panic!()
            }
        })
        .collect())
}
