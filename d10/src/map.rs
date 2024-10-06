use core::str::FromStr;

use anyhow::bail;
use itertools::Itertools;
use libaoc::points::Point2D;

type Pos = Point2D<usize>;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PipeDirection {
    NorthSouth,
    EastWest,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Debug)]
enum Point {
    Pipe(PipeDirection),
    Ground,
    Start,
}

impl TryFrom<char> for Point {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use PipeDirection as PD;
        let res = match value {
            '|' => Self::Pipe(PD::NorthSouth),
            '-' => Self::Pipe(PD::EastWest),
            'L' => Self::Pipe(PD::NorthEast),
            'J' => Self::Pipe(PD::NorthWest),
            '7' => Self::Pipe(PD::SouthWest),
            'F' => Self::Pipe(PD::SouthEast),
            '.' => Self::Ground,
            'S' => Self::Start,
            c => bail!("invalid direction: {c}"),
        };
        Ok(res)
    }
}

pub(crate) struct Map {
    inner: Vec<Vec<Point>>,
    width: usize,
    height: usize,
    start_pos: Pos,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut map = Vec::with_capacity(height);
        for line in s.lines() {
            let line: Vec<Point> = line.chars().map(Point::try_from).try_collect()?;
            map.push(line);
        }

        let start_pos = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .position(|p| matches!(p, Point::Start))
                    .map(|x| Point2D(x, y))
            })
            .unwrap();

        Ok(Self {
            inner: map,
            width,
            height,
            start_pos,
        })
    }
}

impl Map {
    fn adjacent(&self, &p @ Point2D(x, y): &Pos) -> Option<[Pos; 2]> {
        use PipeDirection as PD;

        let north = |Point2D(x, y)| Point2D(x, y - 1);
        let south = |Point2D(x, y)| Point2D(x, y + 1);
        let east = |Point2D(x, y)| Point2D(x + 1, y);
        let west = |Point2D(x, y)| Point2D(x - 1, y);

        #[allow(clippy::match_on_vec_items)]
        match self.inner[y][x] {
            Point::Start => {
                let res = core::iter::empty()
                    // west
                    .chain((x > 0).then(|| west(p)).filter(|p| {
                        matches!(
                            self.inner[p.y()][p.x()],
                            Point::Pipe(PD::NorthEast | PD::EastWest | PD::SouthEast)
                        )
                    }))
                    // east
                    .chain((x < self.width - 1).then(|| east(p)).filter(|p| {
                        matches!(
                            self.inner[p.y()][p.x()],
                            Point::Pipe(PD::NorthWest | PD::EastWest | PD::SouthWest)
                        )
                    }))
                    // north
                    .chain((y > 0).then(|| north(p)).filter(|p| {
                        matches!(
                            self.inner[p.y()][p.x()],
                            Point::Pipe(PD::SouthEast | PD::NorthSouth | PD::SouthWest)
                        )
                    }))
                    // south
                    .chain((y < self.height - 1).then(|| south(p)).filter(|p| {
                        matches!(
                            self.inner[p.y()][p.x()],
                            Point::Pipe(PD::NorthEast | PD::NorthSouth | PD::NorthWest)
                        )
                    }))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("there must be pipes to exactly two sides");

                Some(res)
            }
            Point::Pipe(pd) => {
                let res = match pd {
                    PD::NorthSouth => [north(p), south(p)],
                    PD::EastWest => [east(p), west(p)],
                    PD::NorthWest => [north(p), west(p)],
                    PD::NorthEast => [north(p), east(p)],
                    PD::SouthWest => [south(p), west(p)],
                    PD::SouthEast => [south(p), east(p)],
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
}
