use core::str::FromStr;

use itertools::Itertools;

use crate::game::Game;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

impl Game {
    fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .all(|s| s.red <= MAX_RED && s.green <= MAX_GREEN && s.blue <= MAX_BLUE)
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let games: Box<_> = file.lines().map(Game::from_str).try_collect()?;

    // TODO: change this to `games.into_iter()` in Rust 2024 Edition
    let res = <Box<_> as IntoIterator>::into_iter(games)
        .filter(Game::is_possible)
        .map(|g| g.id)
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 8)]
    #[test_case(REAL => 2239)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
