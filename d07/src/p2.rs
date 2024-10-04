use core::str::FromStr;

use crate::camel_card::{Hand, HandType, Input};
use anyhow::bail;

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

impl From<&Hand<Card>> for HandType {
    fn from(hand: &Hand<Card>) -> Self {
        let freqs = hand.frequencies();
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
                unreachable!("hand can't have any other type: {hand:?}");
            }
        }
    }
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let i = Input::<Card>::from_str(file)?;
    Ok(i.total_winnings())
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
