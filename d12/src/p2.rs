use core::str::FromStr;

use crate::{Record, Spring};

use itertools::Itertools;

impl Record {
    fn unfold(self) -> Self {
        let Self {
            springs,
            description,
        } = self;

        Self {
            springs: vec![springs; 5].join(&Spring::Unknown),
            description: vec![description; 5].concat(),
        }
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let records: Vec<_> = file.lines().map(Record::from_str).try_collect()?;

    let res = records
        .into_iter()
        // .enumerate().inspect(|(i, r)| eprintln!("\nrecord #{i}: {r:?}")).map(|(_, r)| r)
        .map(Record::unfold)
        .map(Record::n_possible_arrangements)
        // .inspect(|n| eprintln!("{n} possible arrangements"))
        .sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 525_152)]
    #[test_case(REAL => 0)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
