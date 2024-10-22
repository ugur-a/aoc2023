use core::str::FromStr;

use crate::contraption::Contraption;

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let c = Contraption::from_str(file)?;

    Ok(c.shine())
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 46)]
    #[test_case(REAL => 8901)]
    fn p1(inp: &str) -> usize {
        super::p1(inp).unwrap()
    }
}
