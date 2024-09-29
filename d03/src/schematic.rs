use core::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Number {
    pub(crate) value: u32,
    pub(crate) start_pos: (usize, usize),
    pub(crate) len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Symbol {
    pub(crate) pos: (usize, usize),
}

#[derive(Debug)]
pub(crate) struct Schematic {
    pub(crate) numbers: Box<[Number]>,
    pub(crate) symbols: HashSet<Symbol>,
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();

        let symbols: HashSet<_> = s
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

        let numbers: Box<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| ((x, y), c)))
            .fold((vec![], None), |(mut acc, mut curr_number), ((x, y), c)| {
                match (&mut curr_number, c.to_digit(10)) {
                    (None, None) => {
                        // no current number and no new one encountered
                        // => no need to do anything
                    }
                    (None, Some(n)) => {
                        // a new number encountered!
                        curr_number = Some(Number {
                            value: n,
                            start_pos: (x, y),
                            len: 1,
                        });
                    }
                    (Some(num), Some(n)) => {
                        // continue building up the number
                        num.value = 10 * num.value + n;
                        num.len += 1;

                        // if this is the last character of the line,
                        // finish up the number immediately
                        if x == width - 1 {
                            acc.push(*num);
                            curr_number = None;
                        }
                    }
                    (Some(num), None) => {
                        // a non-digit encountered - finish up the current number!
                        acc.push(*num);
                        curr_number = None;
                    }
                }
                (acc, curr_number)
            })
            .0
            .into_boxed_slice();

        Ok(Self { numbers, symbols })
    }
}
