use core::str::FromStr;

use anyhow::{bail, ensure, Context};
use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::u32,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub(crate) struct Set {
    pub(crate) red: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32,
}

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) id: u32,
    pub(crate) sets: Vec<Set>,
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for cube in s.split(", ") {
            let (n, color) = cube.split_once(' ').context("cube not space-separated")?;
            let recognized_color = match color {
                "red" => {
                    ensure!(red == 0, "already seen red in this set");
                    &mut red
                }
                "green" => {
                    ensure!(green == 0, "already seen green in this set");
                    &mut green
                }
                "blue" => {
                    ensure!(blue == 0, "already seen blue in this set");
                    &mut blue
                }
                c => bail!("invalid color: {c}"),
            };
            *recognized_color = n.parse()?;
        }
        Ok(Self { red, green, blue })
    }
}

fn set(i: &str) -> IResult<&str, Set> {
    map_res(take_till(|c| c == ';'), Set::from_str)(i)
}

fn sets(i: &str) -> IResult<&str, Vec<Set>> {
    separated_list1(tag("; "), set)(i)
}

fn game(i: &str) -> IResult<&str, Game> {
    map(
        separated_pair(preceded(tag("Game "), u32), tag(": "), sets),
        |(id, sets)| Game { id, sets },
    )(i)
}

impl_from_str_from_nom_parser!(game, Game);
