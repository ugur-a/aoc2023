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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T, // whatever this is
    Joker,
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

impl Hand {
    fn frequencies(self) -> HashMap<Card, u8> {
        let mut res = HashMap::with_capacity(5);
        for c in self.0 {
            *res.entry(c).or_default() += 1;
        }
        res
    }
    fn hand_type(self) -> HandType {
        let freqs: Vec<_> = self.frequencies().into_values().collect();

        match freqs[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] | [4, 1] => HandType::FourOfAKind,
            [2, 3] | [3, 2] => HandType::FullHouse,
            [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => HandType::ThreeOfAKind,
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => HandType::TwoPair,
            [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
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
