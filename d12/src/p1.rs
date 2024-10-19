use core::str::FromStr;

use crate::Record;

use itertools::Itertools;

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let records: Vec<_> = file.lines().map(Record::from_str).try_collect()?;

    let res = records
        .into_iter()
        // .enumerate().inspect(|(i, r)| eprintln!("\nrecord #{i}: {r:?}")).map(|(_, r)| r)
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

    #[test_case(EXAMPLE => 21)]
    #[test_case(REAL => 7084)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
}
