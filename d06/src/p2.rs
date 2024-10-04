use core::str::FromStr;

use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::sheet::Race;

struct Sheet(Race);

fn space_separated_number(i: &str) -> IResult<&str, u64> {
    map_res(
        map(separated_list1(space1, digit1), |n| n.concat()),
        |s: String| s.parse(),
    )(i)
}

fn time(i: &str) -> IResult<&str, u64> {
    preceded(tuple((tag("Time:"), space1)), space_separated_number)(i)
}

fn distance(i: &str) -> IResult<&str, u64> {
    preceded(tuple((tag("Distance:"), space1)), space_separated_number)(i)
}

fn sheet(i: &str) -> IResult<&str, Sheet> {
    map(
        separated_pair(time, newline, distance),
        |(time, distance)| Sheet(Race { time, distance }),
    )(i)
}

impl_from_str_from_nom_parser!(sheet, Sheet);

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let s = Sheet::from_str(file)?;
    let res = s.0.ways_to_win();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 71503)]
    #[test_case(REAL => 36_530_883)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
