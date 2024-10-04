use core::str::FromStr;
use std::collections::HashSet;

use itertools::Itertools;

use crate::card::Card;

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let cards: Box<_> = file.lines().map(Card::from_str).try_collect()?;

    let worths: Box<_> = cards
        .iter()
        .map(|c| HashSet::intersection(&c.your_numbers, &c.winning_numbers).count())
        .collect();

    // how many cards you'll get in total after beginning with this one card
    let mut actual_worths = vec![1u32; cards.len()];
    for i in (0..cards.len()).rev() {
        let add_worth = match worths[i] {
            0 => 0,
            n => actual_worths[i + 1..][..n].iter().sum(),
        };
        actual_worths[i] += add_worth;
    }

    let res = actual_worths.into_iter().sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 30)]
    #[test_case(REAL => 12_648_035)]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
