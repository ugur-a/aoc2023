use core::str::FromStr;

use anyhow::Context;
use libaoc::{
    direction::Direction4 as Direction,
    points::{ManhattanDistance, Point2D},
};
use pathfinding::directed::astar;

use crate::Map;

type Pos = Point2D<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UltraCrucible {
    pos: Pos,
    direction: Direction,
    moves_since_turn: u8,
}

const MAX_MOVES_TILL_TURN: u8 = 10;
const MIN_MOVES_AFTER_TURN: u8 = 4;

impl UltraCrucible {
    fn new(pos: Pos, direction: Direction, moves_since_turn: u8) -> Self {
        Self {
            pos,
            direction,
            moves_since_turn,
        }
    }
    fn successors(&self, map: &Map) -> Vec<(Self, usize)> {
        let &UltraCrucible {
            pos,
            direction,
            moves_since_turn,
        } = self;

        let mut successors = Vec::with_capacity(3);

        // workaround to include the case where the crucible starts looking down
        if pos == Point2D::default() {
            successors.push((UltraCrucible::new(pos, Direction::Down, 0), 0));
        }

        // turn
        if moves_since_turn >= MIN_MOVES_AFTER_TURN {
            for d in [direction.turn_right(), direction.turn_left()] {
                if let Some(p) = map.0.try_go(pos, d) {
                    let cost = map.0[p] as usize;
                    successors.push((UltraCrucible::new(p, d, 1), cost));
                }
            }
        }

        // continue straight
        if moves_since_turn < MAX_MOVES_TILL_TURN {
            if let Some(p) = map.0.try_go(pos, direction) {
                let cost = map.0[p] as usize;
                successors.push((UltraCrucible::new(p, direction, moves_since_turn + 1), cost));
            }
        }

        // eprintln!("\nsucc({self:?}):");
        // for s in &successors {
        //     eprintln!(" {s:?}");
        // }

        successors
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(file)?;

    let start = UltraCrucible::new(Pos::default(), Direction::Right, 0);
    let dest_pos = Point2D(map.0.width() - 1, map.0.height() - 1);

    let (_path, cost) = astar::astar(
        &start,
        |c| c.successors(&map),
        |c| c.pos.manhattan_distance(dest_pos),
        |c| c.pos == dest_pos,
    )
    .context("no path")?;

    Ok(cost)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 94)]
    #[test_case(REAL => 982)]
    fn p2(inp: &str) -> usize {
        super::p2(inp).unwrap()
    }
}
