use core::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

use crate::{Part, Workflow, Workflows};

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let (workflows, parts) = file
        .split_once("\n\n")
        .context("no empty line between workflows and parts")?;

    let workflows = workflows.lines().map(Workflow::try_from).try_collect()?;
    let workflows = Workflows::new(workflows);

    let parts: Vec<_> = parts.lines().map(Part::from_str).try_collect()?;

    let res = parts
        .into_iter()
        .filter(|p| workflows.is_accepted(p))
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 19114)]
    #[test_case(REAL => 319_295)]
    fn p1(inp: &str) -> u32 {
        super::p1(inp).unwrap()
    }
}
