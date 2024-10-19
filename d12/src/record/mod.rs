use core::{cmp::Ordering, fmt::Write, iter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Spring {
    Broken,
    Working,
    Unknown,
}

impl core::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Broken => '#',
            Self::Working => '.',
            Self::Unknown => '?',
        };
        f.write_char(res)
    }
}

type Springs = Vec<Spring>;
type Description = Vec<usize>;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) springs: Springs,
    pub(crate) description: Description,
}

mod parse;

impl Record {
    pub fn n_possible_arrangements(self) -> usize {
        n_possible_arrangements_rec(&self.springs[..], &self.description[..])
    }
}

fn min_len(description: &[usize]) -> usize {
    if description.is_empty() {
        0
    } else {
        description.iter().sum::<usize>() + (description.len() - 1)
    }
}

fn can_place_spring(spring: Spring) -> bool {
    matches!(spring, Spring::Broken | Spring::Unknown)
}

fn n_possible_arrangements_rec(mut springs: &[Spring], mut description: &[usize]) -> usize {
    loop {
        match springs.len().cmp(&min_len(description)) {
            Ordering::Less => {
                // can't fit the rest of the chunks, abort
                return 0;
            }
            Ordering::Equal => {
                let expected_springs: Springs = description
                    .iter()
                    .copied()
                    .map(|chunk_len| vec![Spring::Broken; chunk_len])
                    .collect::<Vec<_>>()
                    .join(&Spring::Working);

                return usize::from(
                    iter::zip(springs, expected_springs)
                        .all(|(&s, e_s)| s == Spring::Unknown || s == e_s),
                );
            }
            Ordering::Greater => {}
        }

        if description.is_empty() && springs.contains(&Spring::Broken) {
            // there's a Spring::Broken to be placed, but we don't have any left
            return 0;
        }

        match springs.iter().copied().position(can_place_spring) {
            Some(0) => {}
            Some(n) => {
                // skip to a place where a spring can be put
                springs = &springs[n..];
                continue;
            }
            None => {
                // no more valid places to put chunks
                return usize::from(description.is_empty());
            }
        }

        // take the next chunk
        let old_description = description;
        let Some(&chunk1_len) = description.first() else {
            // no more chunks
            return 1;
        };
        description = &description[1..];

        if let Some(working_pos) = springs[..chunk1_len]
            .iter()
            .copied()
            .position(|s| !can_place_spring(s))
        {
            // need to skip to after Spring::Working
            return if springs[..working_pos].contains(&Spring::Broken) {
                // can't skip since there is a Spring::Broken before it
                0
            } else {
                springs = &springs[working_pos + 1..];
                description = old_description;
                continue;
            };
        }

        // put the chunk!

        let Some(spring_after_chunk) = springs.get(chunk1_len) else {
            // we're at the end, put the last chunk and finish
            return 1;
        };

        // the arrangements we get if we skip the first spring
        let res2 = if matches!(springs[0], Spring::Broken) {
            // can't skip it since it's Broken
            0
        } else {
            // skip it
            n_possible_arrangements_rec(&springs[1..], old_description)
        };

        if let Spring::Broken = spring_after_chunk {
            // we can't not skip the first spring,
            // otherwise the would-be chunk would be directly followed by a spring
            return res2;
        }

        // there's space after the to-be-placed chunk

        let res1 = n_possible_arrangements_rec(&springs[chunk1_len + 1..], description);

        return res1 + res2;
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::*;
    use test_case::test_case;

    #[test_case("# 1", &[&[0]]; "it works")]
    #[test_case("###.# 3,1", &[&[0,4]]; "no unknowns")]
    #[test_case(".??..??...?##. 1,1,3", &[&[1,5,10], &[1,6,10], &[2,5,10], &[2,6,10]]; "example line 2")]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", &[&[1,3,7,9]]; "example line 3")]
    // from real.txt
    #[test_case("????#?.?..????# 3,4", &[&[2,11], &[3,11]])]
    #[test_case(
        "?????#????#? 2,1,1,1",
        &[
            &[0, 3, 5, 10],
            &[0, 5, 7, 10], &[0, 5, 8, 10],
            &[1, 5, 7, 10], &[1, 5, 8, 10],
            &[2, 5, 8, 10], &[2, 5, 7, 10],
        ]
    )]
    #[test_case(
        "???#??##??#??#?????? 8,3,2",
        &[&[3, 12, 16], &[3, 13, 17], &[3, 12, 18], &[3, 13, 18], &[3, 12, 17], &[0, 9, 13]]
    )]
    #[test_case("#??#?? 3,1", &[]; "want to put a chunk but would need to skip a broken spring")]
    fn possible_cases(inp: &str, answer: &[&[usize]]) {
        let r = Record::from_str(inp).unwrap();
        let cases = r.n_possible_arrangements();

        assert_eq!(answer.len(), cases);
    }
}
