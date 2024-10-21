use crate::{
    hash,
    step::{Operation, Step},
};

use itertools::Itertools;

const N_BOXES: usize = 256;

struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let steps: Vec<_> = (file.strip_suffix('\n').unwrap())
        .split(',')
        .map(Step::try_from)
        .try_collect()?;

    let mut boxes: [Vec<Lens>; N_BOXES] = core::array::from_fn(|_| vec![]);

    for step in steps {
        let Step { label, operation } = step;
        let box_i = hash(label);
        let boks = &mut boxes[box_i as usize];

        match operation {
            Operation::Remove => {
                if let Some(i) = boks.iter().position(|l| l.label == label) {
                    boks.remove(i);
                }
            }
            Operation::Insert(focal_length) => match boks.iter().position(|l| l.label == label) {
                Some(i) => boks[i].focal_length = focal_length,
                None => boks.push(Lens {
                    label,
                    focal_length,
                }),
            },
        }
    }

    let res = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_i, boks)| {
            boks.into_iter()
                .enumerate()
                .map(move |(lens_i, lens)| (box_i + 1) * (lens_i + 1) * lens.focal_length as usize)
        })
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 145)]
    #[test_case(REAL => 229_349)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
