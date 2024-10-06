use core::str::FromStr;

use crate::map::Map;

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(file)?;

    let res = map.find_loop().len() / 2;
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE1: &str = include_str!("../inputs/example1.txt");
    const EXAMPLE2: &str = include_str!("../inputs/example2.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE1 => 4)]
    #[test_case(EXAMPLE2 => 8)]
    #[test_case(REAL => 6806)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
}
