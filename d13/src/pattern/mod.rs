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
enum Mirror {
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

    fn find_mirror(&self) -> Mirror {
        fn are_mirror_opposites(a: &[Vec<Point>], b: &[Vec<Point>]) -> bool {
            core::iter::zip(a.iter().rev(), b).all(|(a, b)| a == b)
        }

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
}
