use anyhow::bail;
use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    IResult,
};

use super::{Pattern, PatternNotes, Point};

impl TryFrom<char> for Point {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            c => bail!("invalid point: {c}"),
        };
        Ok(res)
    }
}

fn point(i: &str) -> IResult<&str, Point> {
    map_res(anychar, Point::try_from)(i)
}

fn pattern(i: &str) -> IResult<&str, Pattern> {
    map(separated_list1(newline, many1(point)), Pattern)(i)
}

impl_from_str_from_nom_parser!(pattern, Pattern);

fn pattern_notes(i: &str) -> IResult<&str, PatternNotes> {
    map(separated_list1(tag("\n\n"), pattern), PatternNotes)(i)
}

impl_from_str_from_nom_parser!(pattern_notes, PatternNotes);
