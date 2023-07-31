use strum_macros::EnumIter; // 0.17.1
use rand::Rng;

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

#[derive(Debug, Clone)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
}

#[derive(Debug)]
pub struct PokerGame {
    pub player_hands: Vec<Hand>,
    pub table_cards: Vec<Card>,
    pub deck_cards: Vec<Card>,
}

impl PokerGame {
    pub fn fill_cards(&mut self) {
        while self.table_cards.len() < 5 {
            let rand_index = rand::thread_rng().gen_range(0..self.deck_cards.len());
            let rand_card = self.deck_cards[rand_index];
            self.deck_cards.remove(rand_index);
            self.table_cards.push(rand_card);
        }
    }
}
