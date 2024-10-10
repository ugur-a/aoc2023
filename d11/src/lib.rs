use core::{marker::PhantomData, str::FromStr};

use anyhow::bail;
use itertools::Itertools;
use libaoc::points::Point2D;

pub mod p1;
pub mod p2;

type Pos = Point2D<usize>;

enum Point {
    Galaxy,
    Space,
}

impl TryFrom<char> for Point {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '#' => Self::Galaxy,
            '.' => Self::Space,
            c => bail!("invalid point: {c}"),
        };
        Ok(res)
    }
}

struct Image<P> {
    inner: Vec<Vec<Point>>,
    pub expanded_rows: Vec<usize>,
    pub expanded_cols: Vec<usize>,
    _marker: PhantomData<P>,
}

impl<P> FromStr for Image<P> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Point::try_from).try_collect())
            .try_collect()?;
        let expanded_rows: Vec<_> = inner
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|p| matches!(p, Point::Space)))
            .map(|(y, _)| y)
            .collect();
        let expanded_cols = (0..inner[0].len())
            .filter(|&x| {
                inner
                    .iter()
                    .map(|row| &row[x])
                    .all(|p| matches!(p, Point::Space))
            })
            .collect();

        Ok(Self {
            inner,
            expanded_rows,
            expanded_cols,
            _marker: PhantomData,
        })
    }
}

impl<P> Image<P> {
    pub fn galaxies(&self) -> impl Iterator<Item = Pos> + '_ {
        self.inner.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, p)| matches!(p, Point::Galaxy))
                .map(move |(x, _)| Point2D(x, y))
        })
    }
}

pub trait Distance {
    type Point;
    fn distance(&self, p1: Self::Point, p2: Self::Point) -> usize;
}
