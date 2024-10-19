use anyhow::bail;
use libaoc::{impl_from_str_from_nom_parser, parse::n};
use nom::{
    character::complete::{anychar, char, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use super::{Description, Record, Spring, Springs};

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '#' => Self::Broken,
            '.' => Self::Working,
            '?' => Self::Unknown,
            c => bail!("invalid spring: {c}"),
        };
        Ok(res)
    }
}

fn spring(i: &str) -> IResult<&str, Spring> {
    map_res(anychar, Spring::try_from)(i)
}

fn springs(i: &str) -> IResult<&str, Springs> {
    many1(spring)(i)
}

fn description(i: &str) -> IResult<&str, Description> {
    separated_list1(char(','), n)(i)
}

fn record(i: &str) -> IResult<&str, Record> {
    map(
        separated_pair(springs, space1, description),
        |(left_half, right_half)| Record {
            springs: left_half,
            description: right_half,
        },
    )(i)
}

impl_from_str_from_nom_parser!(record, Record);
