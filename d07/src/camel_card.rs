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
