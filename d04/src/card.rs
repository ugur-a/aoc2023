use std::collections::HashSet;

use libaoc::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{char, space0, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

pub(crate) struct Card {
    pub(crate) id: u32,
    pub(crate) winning_numbers: HashSet<u32>,
    pub(crate) your_numbers: HashSet<u32>,
}

fn number(i: &str) -> IResult<&str, u32> {
    preceded(space0, u32)(i)
}

fn numbers(i: &str) -> IResult<&str, HashSet<u32>> {
    map(separated_list0(char(' '), number), HashSet::from_iter)(i)
}

fn card(i: &str) -> IResult<&str, Card> {
    map(
        separated_pair(
            preceded(tag("Card"), number),
            tag(": "),
            separated_pair(numbers, tag(" | "), numbers),
        ),
        |(id, (winning_numbers, your_numbers))| Card {
            id,
            winning_numbers,
            your_numbers,
        },
    )(i)
}

impl_from_str_from_nom_parser!(card, Card);
