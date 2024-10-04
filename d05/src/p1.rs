use core::str::FromStr;

use anyhow::Context;
use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

struct Almanac {
    seeds: Vec<u32>,
    seed2soil: Mapping,
    soil2fertilizer: Mapping,
    fertilizer2water: Mapping,
    water2light: Mapping,
    light2temperature: Mapping,
    temperature2humidity: Mapping,
    humidity2location: Mapping,
}

fn seeds(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), separated_list1(char(' '), u32))(i)
}

fn transformation(i: &str) -> IResult<&str, Transformation> {
    map(
        tuple((u32, preceded(char(' '), u32), preceded(char(' '), u32))),
        |(dst_start, src_start, len)| Transformation {
            src_start,
            len,
            dst_start,
        },
    )(i)
}

fn mapping(name: &str) -> impl FnMut(&str) -> IResult<&str, Mapping> + '_ {
    move |i: &str| {
        map(
            preceded(
                tuple((tag(name), tag(" map:"), newline)),
                separated_list1(newline, transformation),
            ),
            Mapping,
        )(i)
    }
}

fn newline2(i: &str) -> IResult<&str, (char, char)> {
    tuple((newline, newline))(i)
}

fn almanac(i: &str) -> IResult<&str, Almanac> {
    map(
        tuple((
            seeds,
            preceded(newline2, mapping("seed-to-soil")),
            preceded(newline2, mapping("soil-to-fertilizer")),
            preceded(newline2, mapping("fertilizer-to-water")),
            preceded(newline2, mapping("water-to-light")),
            preceded(newline2, mapping("light-to-temperature")),
            preceded(newline2, mapping("temperature-to-humidity")),
            preceded(newline2, mapping("humidity-to-location")),
        )),
        |(
            seeds,
            seed2soil,
            soil2fertilizer,
            fertilizer2water,
            water2light,
            light2temperature,
            temperature2humidity,
            humidity2location,
        )| Almanac {
            seeds,
            seed2soil,
            soil2fertilizer,
            fertilizer2water,
            water2light,
            light2temperature,
            temperature2humidity,
            humidity2location,
        },
    )(i)
}

impl_from_str_from_nom_parser!(almanac, Almanac);

impl Almanac {
    fn seed2location(&self, seed: u32) -> u32 {
        seed.map_with(&self.seed2soil)
            .map_with(&self.soil2fertilizer)
            .map_with(&self.fertilizer2water)
            .map_with(&self.water2light)
            .map_with(&self.light2temperature)
            .map_with(&self.temperature2humidity)
            .map_with(&self.humidity2location)
    }
}

struct Mapping(Vec<Transformation>);

struct Transformation {
    src_start: u32,
    len: u32,
    dst_start: u32,
}

trait MapWith {
    fn map_with(self, m: &Mapping) -> Self;
}

impl MapWith for u32 {
    fn map_with(self, m: &Mapping) -> Self {
        for t in &m.0 {
            let Some(d) = self.checked_sub(t.src_start) else {
                continue;
            };
            if d > t.len {
                continue;
            }
            return t.dst_start + d;
        }
        self
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let a = Almanac::from_str(file)?;

    a.seeds
        .iter()
        .map(|s| a.seed2location(*s))
        .min()
        .context("no seeds")
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 35)]
    #[test_case(REAL => 621_354_867)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
