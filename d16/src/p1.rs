pub fn p1(_file: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 46)]
    #[test_case(REAL => ignore)]
    fn p1(inp: &str) -> usize {
        super::p1(inp).unwrap()
    }
}
