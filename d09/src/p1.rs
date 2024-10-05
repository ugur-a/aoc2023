use core::str::FromStr;

use crate::oasis::{OasisReport, ValueHistory};

pub fn p1(file: &str) -> anyhow::Result<i32> {
    let report = OasisReport::from_str(file)?;

    let res = report.0.into_iter().map(ValueHistory::extrapolate).sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 114)]
    #[test_case(REAL => 1_901_217_887)]
    fn test_p1(inp: &str) -> i32 {
        p1(inp).unwrap()
    }
}
