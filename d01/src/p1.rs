use core::{marker::PhantomData, str::FromStr};

use anyhow::Context;
use itertools::Itertools;

use crate::WeirdNumber;

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

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example1.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 142)]
    #[test_case[REAL => 55712]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
