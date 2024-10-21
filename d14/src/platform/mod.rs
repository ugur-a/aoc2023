use core::fmt::{Display, Write};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum MaybeRock {
    None,
    Rounded,
    Cube,
}

impl core::fmt::Debug for MaybeRock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Cube => '#',
            Self::Rounded => 'O',
            Self::None => '.',
        };
        f.write_char(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Platform(pub(crate) Vec<Vec<MaybeRock>>);

mod parse;

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for mr in row {
                write!(f, "{mr:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Platform {
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn rows_mut(&mut self) -> &mut Vec<Vec<MaybeRock>> {
        &mut self.0
    }

    pub(crate) fn tilt_north(&mut self) -> bool {
        let mut changed;
        loop {
            changed = false;
            for y in 1..self.height() {
                for x in 0..self.width() {
                    if matches!(self.0[y][x], MaybeRock::Rounded)
                        && matches!(self.0[y - 1][x], MaybeRock::None)
                    {
                        let (above, below) = self.0.split_at_mut(y);
                        std::mem::swap(&mut above[y - 1][x], &mut below[0][x]);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        changed
    }
    pub(crate) fn tilt_west(&mut self) -> bool {
        let mut changed = false;
        for row in self.rows_mut() {
            for part in row.split_mut(|mr| matches!(mr, MaybeRock::Cube)) {
                // HACK: this will become a hard error once I update to 1.82.0
                // TODO: rm the `allow` and the trait
                #[allow(unstable_name_collisions)]
                if !part.is_sorted_by(|a, b| b <= a) {
                    changed = true;
                }
                part.sort_unstable_by(|a, b| b.cmp(a));
            }
        }
        changed
    }

    pub(crate) fn tilt_south(&mut self) -> bool {
        let mut changed;
        loop {
            changed = false;
            for y in 0..self.height() - 1 {
                for x in 0..self.width() {
                    if matches!(self.0[y][x], MaybeRock::Rounded)
                        && matches!(self.0[y + 1][x], MaybeRock::None)
                    {
                        let (above, below) = self.0.split_at_mut(y + 1);
                        std::mem::swap(&mut above[y][x], &mut below[0][x]);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        changed
    }

    pub(crate) fn tilt_east(&mut self) -> bool {
        let mut changed = false;
        for row in self.rows_mut() {
            for part in row.split_mut(|mr| matches!(mr, MaybeRock::Cube)) {
                // HACK: this will become a hard error once I update to 1.82.0
                // TODO: rm the `allow` and the trait
                #[allow(unstable_name_collisions)]
                if !part.is_sorted() {
                    changed = true;
                }
                part.sort_unstable();
            }
        }
        changed
    }
    pub(crate) fn north_load(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().map(move |mr| match mr {
                    MaybeRock::Rounded => self.height() - y,
                    _ => 0,
                })
            })
            .sum()
    }
}

trait IsSorted
where
    Self::Item: Ord,
{
    type Item;

    fn is_sorted(&self) -> bool {
        self.is_sorted_by(|a, b| a <= b)
    }

    fn is_sorted_by<FN>(&self, f: FN) -> bool
    where
        FN: Fn(&Self::Item, &Self::Item) -> bool;
}

impl<T> IsSorted for [T]
where
    T: Ord,
{
    type Item = T;

    fn is_sorted_by<FN>(&self, f: FN) -> bool
    where
        FN: Fn(&Self::Item, &Self::Item) -> bool,
    {
        self.windows(2).all(|w| f(&w[0], &w[1]))
    }
}
