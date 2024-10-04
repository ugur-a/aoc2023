use core::{cmp::Eq, fmt::Debug, hash::Hash, iter::zip};
use std::collections::HashMap;

use nom::{
    character::complete::{anychar, char, newline, u32},
    combinator::{map, map_res},
    multi::{many_m_n, separated_list0},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Hand<C>([C; 5]);

impl<C> Hand<C>
where
    C: Eq + Hash,
{
    pub(crate) fn frequencies(self) -> HashMap<C, u8> {
        let mut res = HashMap::with_capacity(5);
        for c in self.0 {
            *res.entry(c).or_default() += 1;
        }
        res
    }
}

impl<C> PartialOrd for Hand<C>
where
    C: Eq + Ord + Hash,
    for<'a> &'a Hand<C>: Into<HandType>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for Hand<C>
where
    C: Eq + Ord + Hash,
    for<'a> &'a Hand<C>: Into<HandType>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (std::convert::Into::<HandType>::into(self).cmp(&other.into())).then(self.0.cmp(&other.0))
    }
}

pub(crate) struct Input<C>(pub(crate) Vec<(Hand<C>, u32)>);

impl<C> Input<C>
where
    C: Ord + Hash,
    Hand<C>: Copy,
    for<'a> &'a Hand<C>: Into<HandType>,
{
    pub(crate) fn total_winnings(self) -> u32 {
        let Input(mut i) = self;
        i.sort_unstable_by_key(|(hand, _)| *hand);

        zip(1.., i).map(|(rank, (_, bid))| rank * bid).sum()
    }
}

fn card<C>(i: &str) -> IResult<&str, C>
where
    C: TryFrom<char>,
{
    map_res(anychar, C::try_from)(i)
}

fn hand<C>(i: &str) -> IResult<&str, Hand<C>>
where
    C: Debug + TryFrom<char>,
{
    map(many_m_n(5, 5, card), |cs| Hand(cs.try_into().unwrap()))(i)
}

fn input<C>(i: &str) -> IResult<&str, Input<C>>
where
    C: Debug + TryFrom<char>,
{
    map(
        separated_list0(newline, separated_pair(hand, char(' '), u32)),
        Input,
    )(i)
}

impl<C> std::str::FromStr for Input<C>
where
    C: Debug + TryFrom<char>,
{
    type Err = nom::error::Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::Finish;
        match input(s).finish() {
            Ok((_remaining, object)) => Ok(object),
            Err(nom::error::Error { input, code }) => Err(Self::Err {
                input: input.to_string(),
                code,
            }),
        }
    }
}
