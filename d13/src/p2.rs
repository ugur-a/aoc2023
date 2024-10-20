use core::str::FromStr;

use crate::pattern::{AreMirrorOpposites, PatternNotes, Point};

struct P2;

impl AreMirrorOpposites<P2> for &[Vec<Point>] {
    fn are_mirror_opposites(self, other: Self) -> bool {
        // all the pairs of rows must be exactly the same (0 differences),
        // except for 1 pair, which may have exactly 1 difference
        core::iter::zip(self.iter().rev(), other)
            // map each pair of rows to the number of points that don't match between them
            .map(|(row_a, row_b)| {
                core::iter::zip(row_a.iter(), row_b.iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum::<usize>()
            == 1
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let notes = PatternNotes::<P2>::from_str(file)?;

    Ok(notes.summarize())
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
