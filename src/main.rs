use std::convert::Infallible;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use dialoguer::Select;
use console::Term;
use itertools::Itertools;
use std::io;
use colored::Colorize;

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
    let suits = vec!["♠", "♣", "♥", "♦"];
    let suit_str = Select::new()
        .with_prompt("Select suit: ")
        .items(&suits)
        .interact()
        .unwrap();

    let suit = match suit_str {
        0 => Suit::Spade,
        1 => Suit::Club,
        2 => Suit::Heart,
        3 => Suit::Diamond,
        _ => panic!("Unexpected value: {:?}", suit_str),
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
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six ,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::J,
        10 => Rank::Q,
        11 => Rank::K,
        12 => Rank::A,
        _ => panic!("Unexpected value: {:?}", rank_str),
    };
    rank
}

fn unexpect() {
    println!("Something unexpected occurred.")
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

fn place_card_hand(my_deck: &mut Vec<Card>, my_hands: &mut Vec<Hand>, deal: bool) {
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

    if deal {
        let hand = Hand { card1, card2 };
        my_hands.push(hand);
    }
}

fn get_flop(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>) {
    println!("===First Card===");
    place_card_table(my_deck, my_table);

    println!("===Second Card===");
    place_card_table(my_deck, my_table);

    println!("===Third Card===");
    place_card_table(my_deck, my_table);
}

fn main() {
    let mut my_deck = create_deck();
    let mut my_table = Vec::new();
    let mut my_hands: Vec<Hand> = Vec::new();
    let options = vec!["Add player hand", "Add folded hand", "Add flop", "Quit"];

    let choice = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .interact()
        .unwrap();

    match choice {
        0 => place_card_hand(&mut my_deck, &mut my_hands, true),
        1 => place_card_hand(&mut my_deck, &mut my_hands, false),
        2 => get_flop(&mut my_deck, &mut my_table),
        3 => return,
        _ => unexpect(),
    }

    println!("deck is {:?}\n", my_deck);
    println!("table is {:?}\n", my_table);
    println!("hands are {:?}\n", my_hands);
}
