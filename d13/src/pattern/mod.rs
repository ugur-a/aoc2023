use core::fmt::Write;

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
pub(crate) struct Pattern(pub(crate) Vec<Vec<Point>>);

mod parse;

#[derive(Debug)]
pub(crate) enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

impl Pattern {
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

    pub(crate) fn find_mirror(&self) -> Mirror {
        if let Some(n) = (1..self.height()).find(|&mirror_line| {
            let (above, below) = self.rows().split_at(mirror_line);
            are_mirror_opposites(above, below)
        }) {
            Mirror::Horizontal(n)
        } else if let Some(n) = (1..self.width()).find(|&mirror_line| {
            let cols = self.cols();
            let (left, right) = cols.split_at(mirror_line);
            are_mirror_opposites(left, right)
        }) {
            Mirror::Vertical(n)
        } else {
            panic!("mirror not found")
        }
    }

    fn find_smudged_mirror(&self) -> Mirror {
        if let Some(n) = (1..self.height()).find(|&mirror_line| {
            let (above, below) = self.rows().split_at(mirror_line);
            are_smudged_mirror_opposites(above, below)
        }) {
            Mirror::Horizontal(n)
        } else if let Some(n) = (1..self.width()).find(|&mirror_line| {
            let cols = self.cols();
            let (left, right) = cols.split_at(mirror_line);
            are_smudged_mirror_opposites(left, right)
        }) {
            Mirror::Vertical(n)
        } else {
            panic!("mirror not found")
        }
    }
}

fn are_mirror_opposites(a: &[Vec<Point>], b: &[Vec<Point>]) -> bool {
    core::iter::zip(a.iter().rev(), b).all(|(a, b)| a == b)
}

fn are_smudged_mirror_opposites(rows_a: &[Vec<Point>], rows_b: &[Vec<Point>]) -> bool {
    // all the pairs of rows must be exactly the same (0 differences),
    // except for 1 pair, which may have exactly 1 difference
    core::iter::zip(rows_a.iter().rev(), rows_b)
        // map each pair of rows to the number of points that don't match between them
        .map(|(row_a, row_b)| {
            core::iter::zip(row_a.iter(), row_b.iter())
                .filter(|(a, b)| a != b)
                .count()
        })
        .sum::<usize>()
        == 1
}

#[derive(Debug)]
pub(crate) struct PatternNotes(pub(crate) Vec<Pattern>);

impl PatternNotes {
    pub fn summarize(&self) -> usize {
        self.0
            .iter()
            .map(Pattern::find_mirror)
            .map(|m| match m {
                Mirror::Vertical(n) => n,
                Mirror::Horizontal(n) => 100 * n,
            })
            .sum()
    }

    pub fn summarize_smudged(&self) -> usize {
        self.0
            .iter()
            .map(Pattern::find_smudged_mirror)
            .map(|m| match m {
                Mirror::Vertical(n) => n,
                Mirror::Horizontal(n) => 100 * n,
            })
            .sum()
    }
}
