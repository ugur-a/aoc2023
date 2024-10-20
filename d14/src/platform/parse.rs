use anyhow::bail;
use libaoc::impl_from_str_from_nom_parser;
use nom::{
    character::complete::{anychar, newline},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    IResult,
};

use super::{MaybeRock, Platform};

impl TryFrom<char> for MaybeRock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '.' => Self::None,
            'O' => Self::Rounded,
            '#' => Self::Cube,
            c => bail!("invalid point: {c}"),
        };
        Ok(res)
    }
}

fn maybe_rock(i: &str) -> IResult<&str, MaybeRock> {
    map_res(anychar, MaybeRock::try_from)(i)
}

fn platform(i: &str) -> IResult<&str, Platform> {
    map(separated_list1(newline, many1(maybe_rock)), Platform)(i)
}

impl_from_str_from_nom_parser!(platform, Platform);
