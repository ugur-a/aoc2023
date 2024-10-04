use core::{iter::zip, str::FromStr};

use crate::camel_card::Input;

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let Input(mut i) = Input::from_str(file)?;
    i.sort_unstable_by_key(|(hand, _)| *hand);

    let res = zip(1.., i).map(|(rank, (_, bid))| rank * bid).sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 6440)]
    #[test_case(REAL => 253_313_241)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
