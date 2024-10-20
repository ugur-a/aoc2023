use core::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
pub(crate) struct Platform(pub(crate) Vec<Vec<MaybeRock>>);

mod parse;

impl Platform {
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }

    pub(crate) fn tilt_north(&mut self) {
        loop {
            let mut changed = false;
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
