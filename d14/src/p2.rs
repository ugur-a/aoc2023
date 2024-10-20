use core::str::FromStr;

pub fn p2(_file: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => ignore)]
    #[test_case(REAL => ignore)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
