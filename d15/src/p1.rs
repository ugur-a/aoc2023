use crate::hash;

pub fn p1(file: &str) -> u32 {
    file.strip_suffix('\n').unwrap().split(',').map(hash).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 1320)]
    #[test_case(REAL => 515_495)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp)
    }
}
