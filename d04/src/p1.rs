use core::str::FromStr;
use std::collections::HashSet;

use itertools::Itertools;

use crate::card::Card;

impl Card {
    fn worth(&self) -> u32 {
        let nwinning =
            HashSet::intersection(&self.winning_numbers, &self.your_numbers).count() as u32;
        match nwinning {
            0 => 0,
            _ => 2u32.pow(nwinning - 1),
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let cards: Vec<_> = file.lines().map(Card::from_str).try_collect()?;

    let res = cards.iter().map(Card::worth).sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 13)]
    #[test_case[REAL => 20829]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
