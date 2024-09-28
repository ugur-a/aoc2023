use core::str::FromStr;

use itertools::Itertools;

use crate::game::Game;

impl Game {
    fn power(&self) -> u32 {
        let red = self.sets.iter().map(|s| s.red).max().unwrap_or_default();
        let green = self.sets.iter().map(|s| s.green).max().unwrap_or_default();
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap_or_default();

        red * green * blue
    }
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let games: Box<_> = file.lines().map(Game::from_str).try_collect()?;

    let res = games
        .iter()
        .map(Game::power)
        .inspect(|p| eprintln!("{p}"))
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 2286)]
    #[test_case[REAL => 83435]]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
