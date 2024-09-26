use anyhow::Context;

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let mut res = 0;
    for line in file.lines() {
        let first = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .context("no digits")?;
        let second = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .context("only one digit")?;
        res += 10 * first + second;
    }
    Ok(res)
}

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

    #[test_case(EXAMPLE1 => 142)]
    #[test_case[REAL => 55712]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }

    #[test_case(EXAMPLE2 => 281)]
    #[test_case[REAL => ignore 0]]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
