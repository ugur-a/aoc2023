use core::{marker::PhantomData, str::FromStr};

use anyhow::Context;
use itertools::Itertools;

struct WeirdNumber<P>(u32, PhantomData<P>);

struct P1;
impl FromStr for WeirdNumber<P1> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s
            .chars()
            .find_map(|c| c.to_digit(10))
            .context("no digits")?;
        let second = s
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .context("only one digit")?;
        Ok(Self(10 * first + second, PhantomData))
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    file.lines()
        .map(WeirdNumber::<P1>::from_str)
        .fold_ok(0, |acc, WeirdNumber(n, _)| acc + n)
}

const DIGIT_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct P2;
impl FromStr for WeirdNumber<P2> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::manual_map)]
        let first: u32 = (0..s.len())
            .map(|i| &s[i..])
            .find_map(|s| {
                if let Some(d) = s.chars().next().expect("`i` in bounds").to_digit(10) {
                    Some(d)
                } else if let Some(d) = DIGIT_STRS.into_iter().position(|n| s.starts_with(n)) {
                    Some(d as u32 + 1)
                } else {
                    None
                }
            })
            .context("no digits")?;

        #[allow(clippy::manual_map)]
        let second = (0..=s.len())
            .rev()
            .map(|i| &s[..i])
            .find_map(|s| {
                if let Some(d) = s.chars().next_back().expect("`i` in bounds").to_digit(10) {
                    Some(d)
                } else if let Some(d) = DIGIT_STRS.into_iter().position(|n| s.ends_with(n)) {
                    Some(d as u32 + 1)
                } else {
                    None
                }
            })
            .context("only one digit")?;

        Ok(Self(10 * first + second, PhantomData))
    }
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    file.lines()
        .map(WeirdNumber::<P2>::from_str)
        .fold_ok(0, |acc, WeirdNumber(n, _)| acc + n)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE1: &str = include_str!("../inputs/example1.txt");
    const EXAMPLE2: &str = include_str!("../inputs/example2.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE1 => 142)]
    #[test_case[REAL => 55712]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }

    #[test_case(EXAMPLE2 => 281)]
    #[test_case[REAL => 55413]]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
