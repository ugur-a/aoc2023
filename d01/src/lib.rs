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

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 142)]
    #[test_case[REAL => 55712]]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
