use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use itertools::Itertools;
use std::io;

#[derive(Debug, Copy, Clone, EnumIter)]
enum Suit {
    Diamond,
    Club,
    Spade,
    Heart,
}

#[derive(Debug, Copy, Clone, EnumIter)]
enum Rank {
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

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
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

struct Hand {
    card1: Card,
    card2: Card,
}

struct Flop {
    card1: Card,
    card2: Card,
    card3: Card,
}

fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn main() {
    let my_deck = create_deck();

    let mut players = 0;

    loop {
        println!("Enter the number of players:");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut players)
            .expect("Yo this did not work.");
    }
}