use core::str::FromStr;

use libaoc::points::Point2D;

use crate::contraption::{Beam, Contraption, Direction};

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let c = Contraption::from_str(file)?;

    let w = c.width();
    let h = c.height();

    let res = (0..w)
        .flat_map(|x| {
            [
                Beam {
                    pos: Point2D(x, 0),
                    direction: Direction::Down,
                },
                Beam {
                    pos: Point2D(x, w - 1),
                    direction: Direction::Up,
                },
            ]
        })
        .chain((0..h).flat_map(|y| {
            [
                Beam {
                    pos: Point2D(0, y),
                    direction: Direction::Right,
                },
                Beam {
                    pos: Point2D(h - 1, y),
                    direction: Direction::Left,
                },
            ]
        }))
        .map(|b| c.shine_from(b))
        .max()
        .unwrap_or_default();

    Ok(res)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 51)]
    #[test_case(REAL => 0)]
    fn p2(inp: &str) -> usize {
        super::p2(inp).unwrap()
    }
}
