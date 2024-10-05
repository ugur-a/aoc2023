use crate::map::{Map, MoveDirection};

const START: &str = "AAA";
const DESTINATION: &str = "ZZZ";

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let Map { moves, nodes } = Map::try_from(file)?;

    let mut node = START;
    let mut nmoves = 0;
    for moove in moves.0.into_iter().cycle() {
        if node == DESTINATION {
            break;
        }
        let nexts = &nodes.0[node];
        node = match moove {
            MoveDirection::Left => nexts.left,
            MoveDirection::Right => nexts.right,
        };
        nmoves += 1;
    }
    Ok(nmoves)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE1: &str = include_str!("../inputs/example1.txt");
    const EXAMPLE2: &str = include_str!("../inputs/example2.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE1 => 2)]
    #[test_case(EXAMPLE2 => 6)]
    #[test_case(REAL => 11309)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
