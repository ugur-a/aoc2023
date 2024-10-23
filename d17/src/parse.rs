use core::str::FromStr;

use anyhow::Context;
use itertools::Itertools;
use libaoc::map::Map2D;

use crate::Map;

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .with_context(|| format!("invalid heat loss value: {c}"))
                    })
                    .try_collect()
            })
            .try_collect()
            .unwrap();

        Ok(Self(Map2D::new(v)))
    }
}
