use std::collections::HashSet;

use crate::number::{parse_numbers, Number};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol {
    pos: (usize, usize),
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let height = file.lines().count();
    let width = file.lines().next().unwrap().len();

    let symbols: HashSet<_> = file
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| ((x, y), c)))
        .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
        .flat_map(|((x, y), _)| {
            std::iter::empty()
                .chain((x > 0).then(|| Symbol { pos: (x - 1, y) }))
                .chain((y > 0).then(|| Symbol { pos: (x, y - 1) }))
                .chain((x < width - 1).then_some(Symbol { pos: (x + 1, y) }))
                .chain((y < height - 1).then_some(Symbol { pos: (x, y + 1) }))
                .chain((x > 0 && y > 0).then(|| Symbol {
                    pos: (x - 1, y - 1),
                }))
                .chain((x > 0 && y < width - 1).then(|| Symbol {
                    pos: (x - 1, y + 1),
                }))
                .chain((x < width - 1 && y > 0).then(|| Symbol {
                    pos: (x + 1, y - 1),
                }))
                .chain((x < width - 1 && y < height - 1).then_some(Symbol {
                    pos: (x + 1, y + 1),
                }))
        })
        .collect();

    let numbers = parse_numbers(file);

    let res = numbers
        .iter()
        .filter(
            |&&Number {
                 start_pos: (start_x, start_y),
                 len,
                 ..
             }| {
                (start_x..start_x + len as usize)
                    .map(|x| Symbol { pos: (x, start_y) })
                    .any(|s| symbols.contains(&s))
            },
        )
        .map(|n| n.value)
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 4361)]
    #[test_case(REAL => 536_576)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
