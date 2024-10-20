use core::str::FromStr;

use crate::platform::Platform;

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let mut platform = Platform::from_str(file)?;
    platform.tilt_north();

    Ok(platform.north_load())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 136)]
    #[test_case(REAL => 108_840)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
}
