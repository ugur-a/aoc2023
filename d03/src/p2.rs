use std::collections::HashMap;

use crate::number::{parse_numbers, Number};

#[derive(Debug)]
enum GearStatus {
    Empty,
    One(u32),
    Complete(u32, u32),
    No,
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let height = file.lines().count();
    let width = file.lines().next().unwrap().len();

    let mut gear_statuses: HashMap<_, _> = file
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| ((x, y), c)))
        .filter(|(_, c)| *c == '*')
        .map(|(pos, _)| (pos, GearStatus::Empty))
        .collect();

    let numbers = parse_numbers(file);

    for number in numbers {
        let Number {
            value,
            start_pos: (start_x, y),
            len,
        } = number;

        let end_x = start_x + len as usize - 1;

        let mut neighbours: Vec<_> = std::iter::empty()
            .chain((start_x > 0).then(|| (start_x - 1, y)))
            .chain((end_x < width - 1).then_some((end_x + 1, y)))
            .chain((start_x > 0 && y > 0).then(|| (start_x - 1, y - 1)))
            .chain((start_x > 0 && y < width - 1).then(|| (start_x - 1, y + 1)))
            .chain((end_x < width - 1 && y > 0).then(|| (end_x + 1, y - 1)))
            .chain((end_x < width - 1 && y < height - 1).then_some((end_x + 1, y + 1)))
            .collect();

        if y > 0 {
            neighbours.extend((start_x..=end_x).map(|x| (x, y - 1)));
        }
        if y < height - 1 {
            neighbours.extend((start_x..=end_x).map(|x| (x, y + 1)));
        }

        for neighbour in neighbours {
            if let Some(gear) = gear_statuses.get_mut(&neighbour) {
                *gear = match gear {
                    GearStatus::Empty => GearStatus::One(value),
                    &mut GearStatus::One(v) => GearStatus::Complete(v, value),
                    GearStatus::Complete(..) | GearStatus::No => GearStatus::No,
                };
            }
        }
    }

    let res = gear_statuses
        .into_values()
        .filter_map(|gs| match gs {
            GearStatus::Complete(first, second) => Some(first * second),
            _ => None,
        })
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 467_835)]
    #[test_case[REAL => 75_741_499]]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
