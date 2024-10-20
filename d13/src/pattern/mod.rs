use core::{fmt::Write, marker::PhantomData};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Point {
    Ash,
    Rocks,
}

impl core::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Rocks => '#',
            Self::Ash => '.',
        };
        f.write_char(res)
    }
}

#[derive(Debug)]
pub(crate) struct Pattern<P>(pub(crate) Vec<Vec<Point>>, PhantomData<P>);

mod parse;

#[derive(Debug)]
pub(crate) enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

impl<P> Pattern<P> {
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn rows(&self) -> &Vec<Vec<Point>> {
        &self.0
    }

    fn cols(&self) -> Vec<Vec<Point>> {
        let width = self.width();
        let map = self.0.clone();
        let mut iters = map
            .into_iter()
            .map(IntoIterator::into_iter)
            .collect::<Vec<_>>();
        (0..width)
            .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
            .collect()
    }
}

impl<P> Pattern<P>
where
    for<'a> &'a [Vec<Point>]: AreMirrorOpposites<P>,
{
    pub(crate) fn find_mirror(&self) -> Mirror {
        if let Some(n) = (1..self.height()).find(|&mirror_line| {
            let (above, below) = self.rows().split_at(mirror_line);
            above.are_mirror_opposites(below)
        }) {
            Mirror::Horizontal(n)
        } else if let Some(n) = (1..self.width()).find(|&mirror_line| {
            let cols = self.cols();
            let (left, right) = cols.split_at(mirror_line);
            left.are_mirror_opposites(right)
        }) {
            Mirror::Vertical(n)
        } else {
            panic!("mirror not found")
        }
    }
}

pub(crate) trait AreMirrorOpposites<P> {
    fn are_mirror_opposites(self, other: Self) -> bool;
}

#[derive(Debug)]
pub(crate) struct PatternNotes<P>(pub(crate) Vec<Pattern<P>>);

impl<P> PatternNotes<P>
where
    for<'a> &'a [Vec<Point>]: AreMirrorOpposites<P>,
{
    pub fn summarize(&self) -> usize {
        self.0
            .iter()
            .map(Pattern::<P>::find_mirror)
            .map(|m| match m {
                Mirror::Vertical(n) => n,
                Mirror::Horizontal(n) => 100 * n,
            })
            .sum()
    }
}
