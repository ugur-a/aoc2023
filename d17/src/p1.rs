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
struct Crucible {
    pos: Pos,
    direction: Direction,
    moves_till_turn: u8,
}

const MAX_MOVES_TILL_TURN: u8 = 3;

impl Crucible {
    fn new(pos: Pos, direction: Direction, moves_till_turn: u8) -> Self {
        Self {
            pos,
            direction,
            moves_till_turn,
        }
    }
    fn successors(&self, map: &Map) -> Vec<(Self, usize)> {
        let &Crucible {
            pos,
            direction,
            moves_till_turn,
        } = self;

        let mut successors = Vec::with_capacity(3);

        // turn
        for d in [direction.turn_right(), direction.turn_left()] {
            if let Some(p) = map.0.try_go(pos, d) {
                let cost = map.0[p] as usize;
                successors.push((Crucible::new(p, d, MAX_MOVES_TILL_TURN - 1), cost));
            }
        }

        // continue straight
        if moves_till_turn > 0 {
            if let Some(p) = map.0.try_go(pos, direction) {
                let cost = map.0[p] as usize;
                successors.push((Crucible::new(p, direction, moves_till_turn - 1), cost));
            }
        }

        // eprintln!("\nsucc({self:?}):");
        // for s in &successors {
        //     eprintln!(" {s:?}");
        // }

        successors
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(file)?;

    let start = Crucible::new(Pos::default(), Direction::Right, MAX_MOVES_TILL_TURN);
    let dest_pos = Point2D(map.0.width() - 1, map.0.height() - 1);

    let (_path, cost) = astar::astar(
        &start,
        |c| c.successors(&map),
        |c| c.pos.manhattan_distance(dest_pos),
        |c| c.pos == dest_pos,
    )
    .context("no path")?;

    // for step in _path {
    //     eprintln!("{step:?}");
    // }

    Ok(cost)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 102)]
    #[test_case(REAL => 851)]
    fn p1(inp: &str) -> usize {
        super::p1(inp).unwrap()
    }
}
