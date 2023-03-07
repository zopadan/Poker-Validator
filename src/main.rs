use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use itertools::Itertools;

enum Suit {
    Diamond,
    Club,
    Spade,
    Heart,
}

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

fn create_deck() -> Vec<T> {
    let mut deck = Vec::with_capacity(52);
    for _ in 0..52 {
        vec.push(Card::new());
    }

    let mut counter: u8 = 0;

    for (r, s) in iproduct!(Rank::iter(), Suit::iter()) {
        &deck[counter] = Card(r, s);
        counter += 1;
    }
    deck
}

fn main() {
    let person_one_hand = Hand {
        card1: Card {
            rank: Rank::Q,
            suit: Suit::Heart,
        },
        card2: Card {
            rank: Rank::A,
            suit: Suit::Club,
        },
    };
    let person_two_hand = Hand {
        card1: Card {
            rank: Rank::Five,
            suit: Suit::Diamond,
        },
        card2: Card {
            rank: Rank::K,
            suit: Suit::Spade,
        },
    };

    println!("Value of rank in person one card one: {}", person_one_hand.card1.rank_value())
}