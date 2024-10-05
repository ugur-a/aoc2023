use core::str::FromStr;

use crate::oasis::{OasisReport, ValueHistory};

pub fn p2(file: &str) -> anyhow::Result<i32> {
    let report = OasisReport::from_str(file)?;

    let res = report
        .0
        .into_iter()
        .map(ValueHistory::extrapolate_back)
        .sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 2)]
    #[test_case(REAL => 905)]
    fn test_p2(inp: &str) -> i32 {
        p2(inp).unwrap()
    }
}
