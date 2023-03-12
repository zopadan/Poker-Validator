use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Eq, PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum Suit {
    Diamond,
    Club,
    Spade,
    Heart,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    fn suit_value(&self) -> u8 {
        match self.suit {
            Suit::Diamond => 1,
            Suit::Club => 2,
            Suit::Spade => 3,
            Suit::Heart => 4,
        }
    }

    fn rank_value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::J => 11,
            Rank::Q => 12,
            Rank::K => 13,
            Rank::A => 14,
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
}