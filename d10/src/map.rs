use core::str::FromStr;

use anyhow::bail;
use itertools::Itertools;
use libaoc::points::{two_d::Border2D, Point2D};

type Pos = Point2D<usize>;

trait NewTrait {
    fn north(self) -> Self;
    fn south(self) -> Self;
    fn east(self) -> Self;
    fn west(self) -> Self;
}

impl NewTrait for Pos {
    fn north(self) -> Self {
        Point2D(self.x(), self.y() - 1)
    }
    fn south(self) -> Self {
        Point2D(self.x(), self.y() + 1)
    }
    fn east(self) -> Self {
        Point2D(self.x() + 1, self.y())
    }
    fn west(self) -> Self {
        Point2D(self.x() - 1, self.y())
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PipeDirection {
    NorthSouth,
    EastWest,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

use PipeDirection::EastWest as EW;
use PipeDirection::NorthEast as NE;
use PipeDirection::NorthSouth as NS;
use PipeDirection::NorthWest as NW;
use PipeDirection::SouthEast as SE;
use PipeDirection::SouthWest as SW;

#[derive(Debug, Clone, Copy)]
enum Point {
    Pipe(PipeDirection),
    Ground,
}

impl TryFrom<char> for Point {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '|' => Self::Pipe(NS),
            '-' => Self::Pipe(EW),
            'L' => Self::Pipe(NE),
            'J' => Self::Pipe(NW),
            '7' => Self::Pipe(SW),
            'F' => Self::Pipe(SE),
            '.' => Self::Ground,
            c => bail!("invalid direction: {c}"),
        };
        Ok(res)
    }
}

impl Point {
    pub fn points_west(self) -> bool {
        matches!(self, Self::Pipe(NW | SW | EW),)
    }

    pub fn points_east(self) -> bool {
        matches!(self, Self::Pipe(NE | EW | SE))
    }

    pub fn points_south(self) -> bool {
        matches!(self, Self::Pipe(SE | NS | SW))
    }

    pub fn points_north(self) -> bool {
        matches!(self, Self::Pipe(NE | NS | NW))
    }

    fn turn_clockwise(self) -> Self {
        match self {
            Self::Ground => self,
            Self::Pipe(pd) => {
                let pd = match pd {
                    NS => EW,
                    EW => NS,

                    NW => NE,
                    NE => SE,
                    SE => SW,
                    SW => NW,
                };
                Self::Pipe(pd)
            }
        }
    }
}

pub(crate) struct Map {
    inner: Vec<Vec<Point>>,
    start_pos: Pos,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let start_pos = s
            .lines()
            .enumerate()
            .find_map(|(y, line)| line.chars().position(|p| p == 'S').map(|x| Point2D(x, y)))
            .unwrap();

        let pipe_under_start = {
            // NOTE: can't use `Map::get` since the instance is not constructed yet
            let get = |Point2D(x, y)| -> Point {
                s.lines()
                    .nth(y)
                    .unwrap()
                    .chars()
                    .nth(x)
                    .unwrap()
                    .try_into()
                    .unwrap()
            };

            let p @ Point2D(x, y) = start_pos;

            let pipe_to_west = x > 0 && get(p.west()).points_east();
            let pipe_to_east = x < width - 1 && get(p.east()).points_west();
            let pipe_to_north = y > 0 && get(p.north()).points_south();
            let pipe_to_south = y < height - 1 && get(p.south()).points_north();

            let pd = match (pipe_to_west, pipe_to_east, pipe_to_north, pipe_to_south) {
                (true, true, false, false) => EW,
                (true, false, true, false) => NW,
                (true, false, false, true) => SW,
                (false, true, true, false) => NE,
                (false, true, false, true) => SE,
                (false, false, true, true) => NS,
                _ => unreachable!("there must be pipes to exactly two sides"),
            };

            Point::Pipe(pd)
        };

        let mut map = Vec::with_capacity(height);
        for (y, line) in s.lines().enumerate() {
            let line: Vec<Point> = line
                .char_indices()
                .map(|(x, c)| {
                    if Point2D(x, y) == start_pos {
                        Ok(pipe_under_start)
                    } else {
                        Point::try_from(c)
                    }
                })
                .try_collect()?;
            map.push(line);
        }

        Ok(Self {
            inner: map,
            start_pos,
        })
    }
}

impl Map {
    fn get(&self, &Point2D(x, y): &Pos) -> &Point {
        &self.inner[y][x]
    }

    fn adjacent(&self, &p: &Pos) -> Option<[Pos; 2]> {
        match self.get(&p) {
            Point::Pipe(pd) => {
                let res = match pd {
                    NS => [p.north(), p.south()],
                    EW => [p.east(), p.west()],
                    NW => [p.north(), p.west()],
                    NE => [p.north(), p.east()],
                    SW => [p.south(), p.west()],
                    SE => [p.south(), p.east()],
                };
                Some(res)
            }
            Point::Ground => None,
        }
    }

    pub(crate) fn find_loop(&self) -> Vec<Pos> {
        let mut looop = vec![];

        let mut curr = self.start_pos;
        // NOTE: `[0]` is arbitrary here. It amounts to the direction in which
        // we will be traversing the loop
        let mut next = self
            .adjacent(&curr)
            .expect("pipes in the loop only ever lead to other pipes")[0];

        loop {
            let curr_and_next_next = self
                .adjacent(&next)
                .expect("pipes in the loop only ever lead to other pipes");

            let next_next = if curr_and_next_next[0] == curr {
                curr_and_next_next[1]
            } else {
                curr_and_next_next[0]
            };

            if next_next == self.start_pos {
                looop.extend([curr, next, next_next]);
                break;
            }

            looop.push(curr);
            (curr, next) = (next, next_next);
        }
        looop
    }

    /// counts the number of times a horizontal line from `point` crosses the loop
    /// if the number is even, `point` is outside the loop
    /// the opposite is not necessarily true - need to check vertically, too
    ///  NOTE: `Pos` in the iterator is really just for debugging
    fn is_inside_loop_right(points_right: impl Iterator<Item = (Pos, Point)>) -> bool {
        enum EnteredFrom {
            Up,
            Down,
        }
        enum State {
            NotOnLoop,
            OnLoop(EnteredFrom),
        }
        let mut state = State::NotOnLoop;

        let mut n_loop_crosses = 0;

        for (pos, p) in points_right {
            match (&state, p) {
                (State::NotOnLoop, Point::Ground) => {
                    // no change in status
                }
                (State::NotOnLoop, Point::Pipe(pd)) => {
                    match pd {
                        NE => {
                            state = State::OnLoop(EnteredFrom::Up);
                        }
                        SE => {
                            state = State::OnLoop(EnteredFrom::Down);
                        }
                        NW | SW | EW => {
                            unreachable!("invalid loop: entered pipe {p:?} from air at {pos:?}")
                        }
                        NS => {
                            // directly crossed the loop
                            n_loop_crosses += 1;
                        }
                    }
                }
                (State::OnLoop(_), Point::Ground) => {
                    unreachable!("invalid loop: pipe {p:?} exits to air at {pos:?}")
                }
                (State::OnLoop(ef), Point::Pipe(pd)) => {
                    match (ef, pd) {
                        (EnteredFrom::Up, NW) | (EnteredFrom::Down, SW) => {
                            // squeezed along the loop: L-J / F-7
                            // nothing crossed, just exit the loop
                            state = State::NotOnLoop;
                        }
                        (_, EW) => {
                            // continue along the loop
                        }
                        (EnteredFrom::Up, SW) | (EnteredFrom::Down, NW) => {
                            // crossed the loop after going along it: L-7 / F-J
                            n_loop_crosses += 1;
                            state = State::NotOnLoop;
                        }
                        (_, NE | NS | SE) => {
                            unreachable!("invalid loop: pipe {p:?} ended unexpectedly at {pos:?}")
                        }
                    }
                }
            }
        }

        n_loop_crosses % 2 == 1
    }

    pub fn is_inside_loop(
        &self,
        looop: &[Pos],
        border: &Border2D<usize>,
        point @ Point2D(x, y): Pos,
    ) -> bool {
        if looop.contains(&point) {
            return false;
        }

        ({
            // east
            Self::is_inside_loop_right(
                (x + 1..=border.right)
                    .map(|x| Point2D(x, y))
                    .filter(|p| looop.contains(p))
                    .map(|p| (p, *self.get(&p))),
            )
        }) && {
            // north
            Self::is_inside_loop_right(
                (border.top..y)
                    .rev()
                    .map(|y| Point2D(x, y))
                    .filter(|p| looop.contains(p))
                    .map(|p| (p, self.get(&p).turn_clockwise())),
            )
        } && {
            // west
            Self::is_inside_loop_right(
                (border.left..x)
                    .rev()
                    .map(|x| Point2D(x, y))
                    .filter(|p| looop.contains(p))
                    .map(|p| (p, self.get(&p).turn_clockwise().turn_clockwise())),
            )
        } && {
            // south
            Self::is_inside_loop_right(
                (y..=border.down)
                    .map(|y| Point2D(x, y))
                    .filter(|p| looop.contains(p))
                    .map(|p| {
                        (
                            p,
                            self.get(&p)
                                .turn_clockwise()
                                .turn_clockwise()
                                .turn_clockwise(),
                        )
                    }),
            )
        }
    }
}
