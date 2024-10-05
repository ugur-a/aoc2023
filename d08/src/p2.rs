use crate::map::{Map, MoveDirection};
use num::Integer;

pub fn p2(file: &str) -> anyhow::Result<u64> {
    let is_start = |name: &&str| name.ends_with('A');
    let is_end = |name: &str| name.ends_with('Z');

    let Map { moves, nodes } = Map::try_from(file)?;

    let curr_nodes: Vec<_> = nodes.0.keys().copied().filter(is_start).collect();

    let times_to_reach_end = curr_nodes.into_iter().map(|mut node| {
        let mut nmoves = 0;
        for moove in moves.0.iter().cycle() {
            if is_end(node) {
                break;
            }

            let nexts = &nodes.0[node];
            node = match moove {
                MoveDirection::Left => nexts.left,
                MoveDirection::Right => nexts.right,
            };

            nmoves += 1;
        }
        nmoves
    });

    let res = times_to_reach_end.reduce(|acc, n| acc.lcm(&n)).unwrap();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example3.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 6)]
    #[test_case(REAL => 13_740_108_158_591)]
    fn test_p2(inp: &str) -> u64 {
        p2(inp).unwrap()
    }
}
