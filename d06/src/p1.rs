use core::str::FromStr;

use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::sheet::Race;

struct Sheet(Vec<Race>);

fn times(i: &str) -> IResult<&str, Vec<u64>> {
    preceded(tuple((tag("Time:"), space1)), separated_list1(space1, u64))(i)
}

fn distances(i: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, u64),
    )(i)
}

fn sheet(i: &str) -> IResult<&str, Sheet> {
    map(separated_pair(times, newline, distances), |(ts, ds)| {
        let res = std::iter::zip(ts, ds)
            .map(|(time, distance)| Race { time, distance })
            .collect();
        Sheet(res)
    })(i)
}

impl_from_str_from_nom_parser!(sheet, Sheet);

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let s = Sheet::from_str(file)?;
    let res = s.0.iter().map(Race::ways_to_win).product();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 288)]
    #[test_case[REAL => 512_295]]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
}
