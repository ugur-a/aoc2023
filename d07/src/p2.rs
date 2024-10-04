use core::{iter::zip, str::FromStr};
use std::collections::HashMap;

use anyhow::bail;
use libaoc::impl_from_str_from_nom_parser;
use nom::{
    character::complete::{anychar, char, newline, u32},
    combinator::{map, map_res},
    multi::{many_m_n, separated_list0},
    sequence::separated_pair,
    IResult,
};

use crate::camel_card::HandType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T, // whatever this is
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            c => bail!("invalid card: {c}"),
        };
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Hand([Card; 5]);

impl Hand {
    fn frequencies(self) -> HashMap<Card, u8> {
        let mut res = HashMap::with_capacity(5);
        for c in self.0 {
            *res.entry(c).or_default() += 1;
        }
        res
    }
    fn hand_type(self) -> HandType {
        let freqs = self.frequencies();
        let njoker = *freqs.get(&Card::Joker).unwrap_or(&0);

        match freqs.into_values().collect::<Vec<_>>()[..] {
            [5] => match njoker {
                0 | 5 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            [1, 4] | [4, 1] => match njoker {
                1 | 4 => HandType::FiveOfAKind,
                0 => HandType::FourOfAKind,
                _ => unreachable!(),
            },
            [2, 3] | [3, 2] => match njoker {
                2 | 3 => HandType::FiveOfAKind,
                0 => HandType::FullHouse,
                _ => unreachable!(),
            },
            [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => match njoker {
                1 | 3 => HandType::FourOfAKind,
                0 => HandType::ThreeOfAKind,
                _ => unreachable!(),
            },
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => match njoker {
                2 => HandType::FourOfAKind,
                1 => HandType::FullHouse,
                0 => HandType::TwoPair,
                _ => unreachable!(),
            },
            [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => match njoker {
                1 | 2 => HandType::ThreeOfAKind,
                0 => HandType::OnePair,
                _ => unreachable!(),
            },
            [1, 1, 1, 1, 1] => match njoker {
                1 => HandType::OnePair,
                0 => HandType::HighCard,
                _ => unreachable!(),
            },
            _ => {
                unreachable!("hand can't have any other type: {self:?}");
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type().cmp(&other.hand_type())).then(self.0.cmp(&other.0))
    }
}

pub(crate) struct Input(pub(crate) Vec<(Hand, u32)>);

fn card(i: &str) -> IResult<&str, Card> {
    map_res(anychar, Card::try_from)(i)
}

fn hand(i: &str) -> IResult<&str, Hand> {
    map(many_m_n(5, 5, card), |cs| Hand(cs.try_into().unwrap()))(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    map(
        separated_list0(newline, separated_pair(hand, char(' '), u32)),
        Input,
    )(i)
}

impl_from_str_from_nom_parser!(input, Input);

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let Input(mut i) = Input::from_str(file)?;
    i.sort_unstable_by_key(|(hand, _)| *hand);

    let res = zip(1.., i).map(|(rank, (_, bid))| rank * bid).sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 5905)]
    #[test_case[REAL => 253_362_743]]
    fn test_p2(inp: &str) -> u32 {
        p2(inp).unwrap()
    }
}
