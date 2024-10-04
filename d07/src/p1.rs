use core::str::FromStr;

use anyhow::bail;

use crate::camel_card::{Hand, HandType, Input};

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

impl From<&Hand<Card>> for HandType {
    fn from(hand: &Hand<Card>) -> Self {
        let freqs: Vec<_> = hand.frequencies().into_values().collect();

        match freqs[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] | [4, 1] => HandType::FourOfAKind,
            [2, 3] | [3, 2] => HandType::FullHouse,
            [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => HandType::ThreeOfAKind,
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => HandType::TwoPair,
            [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => {
                unreachable!("hand can't have any other type: {hand:?}");
            }
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let i = Input::<Card>::from_str(file)?;
    Ok(i.total_winnings())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 6440)]
    #[test_case(REAL => 253_313_241)]
    fn test_p1(inp: &str) -> u32 {
        p1(inp).unwrap()
    }
}
