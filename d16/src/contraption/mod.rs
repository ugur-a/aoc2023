use std::collections::HashSet;

use libaoc::{map::Map2D, points::Point2D};

type Pos = Point2D<usize>;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MaybeMirror {
    Not,
    Slash,
    Backslash,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: Pos,
    direction: Direction,
}

pub(crate) struct Contraption(Map2D<MaybeMirror>);

impl Contraption {
    pub(crate) fn shine(&self) -> usize {
        let try_move = |beam: Beam, move_direction| {
            let Point2D(x, y) = beam.pos;

            let pos = match move_direction {
                Direction::Left => (x > 0).then(|| Point2D(x - 1, y)),
                Direction::Up => (y > 0).then(|| Point2D(x, y - 1)),
                Direction::Right => (x + 1 < self.0.width()).then_some(Point2D(x + 1, y)),
                Direction::Down => (y + 1 < self.0.height()).then_some(Point2D(x, y + 1)),
            };

            pos.map(|pos| Beam {
                pos,
                direction: move_direction,
            })
        };

        let init_pos = Point2D::default();
        let beam = Beam {
            pos: init_pos,
            direction: Direction::Right,
        };

        let mut beams = vec![beam];
        let mut visited = HashSet::from([beam]);
        while !beams.is_empty() {
            let mut new_beams = vec![];

            // eprintln!("beams: {beams:?}");

            for beam in beams {
                let Beam { pos, direction } = beam;
                let new_directions: &[Direction] = match (direction, self.0[pos]) {
                    // continue straight
                    (_, MaybeMirror::Not)
                    | (Direction::Right | Direction::Left, MaybeMirror::Horizontal)
                    | (Direction::Up | Direction::Down, MaybeMirror::Vertical) => &[direction],

                    // reflect
                    (Direction::Right, MaybeMirror::Slash)
                    | (Direction::Left, MaybeMirror::Backslash) => &[Direction::Up],

                    (Direction::Right, MaybeMirror::Backslash)
                    | (Direction::Left, MaybeMirror::Slash) => &[Direction::Down],

                    (Direction::Up, MaybeMirror::Slash)
                    | (Direction::Down, MaybeMirror::Backslash) => &[Direction::Right],

                    (Direction::Up, MaybeMirror::Backslash)
                    | (Direction::Down, MaybeMirror::Slash) => &[Direction::Left],

                    // split
                    (Direction::Up | Direction::Down, MaybeMirror::Horizontal) => {
                        &[Direction::Left, Direction::Right]
                    }
                    (Direction::Left | Direction::Right, MaybeMirror::Vertical) => {
                        &[Direction::Up, Direction::Down]
                    }
                };

                for direction in new_directions {
                    if let Some(new_beam) = try_move(beam, *direction) {
                        new_beams.push(new_beam);
                    }
                }
            }

            let new_beams = new_beams
                .into_iter()
                .filter(|b| !visited.contains(b))
                .collect();

            // eprintln!("new beams: {new_beams:?}\n");
            beams = new_beams;

            visited.extend(beams.clone());
        }

        /*
        let mut v = vec![vec![false; self.0.width()]; self.0.height()];
        for &Beam {
            pos: Point2D(x, y), ..
        } in &visited
        {
            v[y][x] = true;
        }

        for row in v {
            for c in row {
                eprint!("{}", if c { '#' } else { '.' });
            }
            eprintln!();
        }

        eprintln!("{visited:?}");
        */

        let visited: HashSet<_> = visited.into_iter().map(|b| b.pos).collect();
        visited.len()
    }
}
