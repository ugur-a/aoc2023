use core::str::FromStr;

use libaoc::map::Map2D;

use super::{Contraption, MaybeMirror};

impl TryFrom<char> for MaybeMirror {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Not,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            c => anyhow::bail!("invalid mirror/point: {c}"),
        })
    }
}

impl FromStr for Contraption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = Map2D::from_str(s)?;

        Ok(Self(map))
    }
}
