use core::str::FromStr;

use crate::pattern::PatternNotes;

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let notes = PatternNotes::from_str(file)?;

    Ok(notes.summarize_smudged())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 400)]
    #[test_case(REAL => 36474)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
