use core::str::FromStr;

use crate::pattern::PatternNotes;

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let notes = PatternNotes::from_str(file)?;

    Ok(notes.summarize())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 405)]
    #[test_case(REAL => 30158)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
}
