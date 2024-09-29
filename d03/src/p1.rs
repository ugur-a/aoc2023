use core::str::FromStr;

use crate::schematic::{Number, Schematic, Symbol};

impl Schematic {
    fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.numbers
            .iter()
            .filter(
                |&&Number {
                     start_pos: (start_x, start_y),
                     len,
                     ..
                 }| {
                    (start_x..start_x + len as usize)
                        .map(|x| Symbol { pos: (x, start_y) })
                        .any(|s| self.symbols.contains(&s))
                },
            )
            .map(|n| n.value)
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let s = Schematic::from_str(file)?;

    let res = s.part_numbers().sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 4361)]
    #[test_case[REAL => 536_576]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
