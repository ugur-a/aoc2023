use core::marker::PhantomData;

use anyhow::bail;
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

fn pattern<P>(i: &str) -> IResult<&str, Pattern<P>> {
    map(separated_list1(newline, many1(point)), |p| {
        Pattern(p, PhantomData)
    })(i)
}

impl<P> core::str::FromStr for Pattern<P> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (remaining, object) = pattern(s).map_err(|e| anyhow::anyhow!("{e}"))?;
        anyhow::ensure!(remaining.is_empty() || remaining == "\n");
        Ok(object)
    }
}

fn pattern_notes<P>(i: &str) -> IResult<&str, PatternNotes<P>> {
    map(separated_list1(tag("\n\n"), pattern), PatternNotes)(i)
}

impl<P> core::str::FromStr for PatternNotes<P> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (remaining, object) = pattern_notes(s).map_err(|e| anyhow::anyhow!("{e}"))?;
        anyhow::ensure!(remaining.is_empty() || remaining == "\n");
        Ok(object)
    }
}
