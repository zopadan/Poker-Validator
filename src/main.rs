use std::convert::Infallible;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use dialoguer::Select;
use console::Term;
use itertools::Itertools;
use std::io;

use Poker_Validator::{Card, Rank, Suit, Hand};


fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn get_suit() -> Suit {
    let suits = vec!["Spades", "Clubs", "Hearts", "Diamonds"];
    let suit_str = Select::new()
        .with_prompt("Select suit: ")
        .items(&suits)
        .interact()
        .unwrap();

    let suit = match suit_str {
        "Spades" => Suit::Spade,
        "Clubs" => Suit::Club,
        "Hearts" => Suit::Heart,
        "Diamonds" => Suit::Diamond,
    };
    suit
}

fn get_rank() -> Rank {
    let ranks = [
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "10",
        "J",
        "Q",
        "K",
        "A", ];

    let rank_str = Select::new()
        .with_prompt("Select rank: ")
        .items(&ranks)
        .interact()
        .unwrap();

    let rank = match rank_str {
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six ,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "10" => Rank::Ten,
        "J" => Rank::J,
        "Q" => Rank::Q,
        "K" => Rank::K,
        "A" => Rank::A,
    };
    rank
}

fn remove_card(my_deck: &mut Vec<Card>, card: Card) {
    let card_index = my_deck.iter().position(|&c| c == card).unwrap();
    my_deck.remove(card_index);
}

fn place_card_table(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>) {
    let suit = get_suit();
    let rank = get_rank();
    let card = Card { rank, suit };
    remove_card(my_deck, card);
    my_table.push(card);
}

fn place_card_hand(my_deck: &mut Vec<Card>, my_hands: &mut Vec<Hand>) {
    println!("===First Card===");
    let suit1 = get_suit();
    let rank1 = get_rank();
    let card1 = Card { rank: rank1, suit: suit1 };
    print!("===Second Card===");
    let suit2 = get_suit();
    let rank2 = get_rank();
    let card2 = Card { rank: rank2, suit: suit2 };
    remove_card(my_deck, card1);
    remove_card(my_deck, card2);

    let hand = Hand { card1, card2 };
    my_hands.push(hand);
}

fn get_flop(my_deck: &mut Vec<Card>) {
    println!("===First Card===");
    place_card(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>);

    println!("===Second Card===");
    place_card(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>);

    println!("===Third Card===");
    place_card(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>);
}

fn main() {
    let mut my_deck = create_deck();
    let mut my_table = Vec::new();
    let mut my_hands: Vec<Hand> = Vec::new();
    let options = vec!["Add player hand", "Add folded hand", "Add flop", "Quit"];

    let mut choice = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .interact()
        .unwrap();

    match choice {
        "Add player hand".parse().unwrap() => place_card(&mut my_deck, &mut my_table),
        "Add folded hand".parse().unwrap() => get_hand(&mut my_deck),
        "Add flop".parse().unwrap() => get_flop(&mut my_deck),
        "Quit".parse().unwrap() => Ok (()),
    };

    match choice {
        "Add player hand".parse().unwrap() => hands.push(item),
        "Add folded hand".parse().unwrap() => folds.push(item),
        "Add flop".parse().unwrap() => ,
    }
}
