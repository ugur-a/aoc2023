use core::str::FromStr;

use itertools::Itertools;
use libaoc::points::Point2D;

type Pos = Point2D<usize>;

use crate::{Distance, Image};

struct P2;

const EXPANSION_RATE: usize = 1_000_000;

impl Distance for Image<P2> {
    type Point = Pos;
    fn distance(&self, Point2D(x1, y1): Pos, Point2D(x2, y2): Pos) -> usize {
        let (x1, x2) = (core::cmp::min(x1, x2), core::cmp::max(x1, x2));
        // NOTE: `-1` here is because (each) column already gets counted once in `x2 - x1`
        let dx = (x2 - x1)
            + (EXPANSION_RATE - 1)
                * (x1 + 1..x2)
                    .filter(|x| self.expanded_cols.contains(x))
                    .count();

        let (y1, y2) = (core::cmp::min(y1, y2), core::cmp::max(y1, y2));
        let dy = (y2 - y1)
            + (EXPANSION_RATE - 1)
                * (y1 + 1..y2)
                    .filter(|y| self.expanded_rows.contains(y))
                    .count();

        dx + dy
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let img = Image::<P2>::from_str(file)?;
    let res = img
        .galaxies()
        .combinations(2)
        .map(|g_pair| img.distance(g_pair[0], g_pair[1]))
        .sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 82_000_210)]
    #[test_case(REAL => 544_723_432_977)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
