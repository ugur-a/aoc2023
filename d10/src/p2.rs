pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE1: &str = include_str!("../inputs/example1.txt");
    const EXAMPLE2: &str = include_str!("../inputs/example2.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE1 => ignore)]
    #[test_case(EXAMPLE2 => ignore)]
    #[test_case(REAL => ignore)]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
