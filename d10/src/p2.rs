use core::str::FromStr;

use crate::map::Map;
use libaoc::points::{
    two_d::{min_enclosing_rectangle, Border2D},
    Point2D,
};

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(file)?;
    // eprintln!("{map}");

    let looop = map.find_loop();

    let border @ Border2D {
        left,
        right,
        down,
        top,
    } = min_enclosing_rectangle(looop.iter(), looop.iter());

    let res = (top..=down)
        .flat_map(|y| (left..=right).map(move |x| Point2D(x, y)))
        .filter(|&p| map.is_inside_loop(&looop, &border, p))
        .count();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE3: &str = include_str!("../inputs/example3.txt");
    const EXAMPLE4: &str = include_str!("../inputs/example4.txt");
    const EXAMPLE5: &str = include_str!("../inputs/example5.txt");
    const EXAMPLE6: &str = include_str!("../inputs/example6.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE3 => 4)]
    #[test_case(EXAMPLE4 => 4)]
    #[test_case(EXAMPLE5 => 8)]
    #[test_case(EXAMPLE6 => 10)]
    #[test_case(REAL => 449)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
