use std::convert::Infallible;
use std::intrinsics::raw_eq;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use dialoguer::Select;
use itertools::Itertools;
use std::io;
use rand::distributions::Open01;
use rand::Rng;

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

fn get_rank(suit: Suit, my_deck: &mut Vec<Card>) -> Rank {
    //Need to: iterate through the deck.  For each instance of a card where Card.suit == suit,
    // we will need to render a rank string.  This will ensure that the user never gets the option
    // to deal a card twice.

    let mut ranks = Vec::new();

    for card in my_deck {
        if card.suit == suit {
            let addable_rank = match card.rank {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::J => "J",
                Rank::Q => "Q",
                Rank::K => "K",
                Rank::A => "A",
            };
            ranks.push(addable_rank);
        }
    }

    let rank_index = Select::new()
        .with_prompt("Select rank: ")
        .items(&ranks)
        .interact()
        .unwrap();

    let rank_str = ranks[rank_index];

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
        _ => panic!("Unexpected value: {:?}", rank_str),
    };
    rank
}

fn get_card() -> Card {
    let suit = get_suit();
    let rank = get_rank(suit, my_deck);
    let card = Card { rank, suit };
    card
}

fn move_cards(my_deck: &mut Vec<Card>, cards: Vec<Card>, destination: Option<&mut Vec<Card>>) {
    for card in cards {
        let card_index = my_deck.iter().position(|&c| c == card).unwrap();
        my_deck.remove(card_index);
        match destination {
            Some(destination) => destination.push(card),
            None => Ok(()),
        }
    }
}

fn handler() -> Vec<Card> {
    let hand = Hand { cards: None };

}

fn add_turn_river(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>) {
    let i = 0;
    while i < 2 {
        let rand_index = rand::thread_rng().gen_range(0..my_deck.len());
        let rand_card = my_deck[rand_index];
        my_deck.remove(rand_index);
        my_table.push(rand_card);
    }
}

fn calculate(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>, my_hands: &mut Vec<Hand>) {
    add_turn_river(my_deck, my_table);
    for item in my_hands.iter() {
        let mut cards = Vec::new();
        for card in item {
            cards.push(card);
        }

        let combinations = cards
        .into_iter()
            .cartesian_product(my_table.into_iter())
            .combinations(5)
            .map(|v| v.into_iter().map(|(_, val)| val).collect::<Vec<_>>())
            .collect::<Vec<_>>();

    }
}

fn refresh(my_deck: &mut Vec<Card>, my_table: &mut Vec<Card>, my_hands: &mut Vec<Hand>) {
    *my_deck = create_deck();
    my_table.clear();
    my_hands.clear();
}

fn main() {
    let mut my_deck = create_deck();
    let mut my_table = Vec::new();
    let mut my_hands: [Hand; 9];

    loop {
        // NOTE: by doing this in this way, we are re-generating the options vector with each loop
        // stubbing this for potential future optimization.
        // Possibly reset options by removing calc odds and start over at the end of each loop.
        let mut options = vec![
            "Add player hand", "Add folded hand",
            "Add flop", "Quit (break and debug print)",
        ];
        let calculable: bool = my_hands.len() >= 2 && my_table.len() == 3 && my_deck.len() < 52;
        let refreshable: bool = my_deck.len() < 52;

        if calculable {
            options.insert(options.len() - 1, "Calculate Odds!");
        }

        if refreshable {
            options.insert(options.len() - 1, "Start Over");
        }

        let choice = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .interact()
            .unwrap();

        let choice_str = options[choice];

        match choice_str {
            "Add player hand" => move_cards(&mut my_deck, vec![get_card(), get_card()], ),
            "Add folded hand" => move_cards(&mut my_deck, vec![get_card(), get_card()], None),
            "Add flop" => move_cards(&mut my_deck, vec![get_card(), get_card(), get_card()], Some(&mut my_table)),
            "Start Over" => refresh(&mut my_deck, &mut my_table, &mut my_hands),
            "Calculate Odds!" => calculate(&mut my_deck, &mut my_table, &mut my_hands),
            "Quit (break and debug print)" => break,
            _ => panic!("Unexpected value {:?}", choice),
        }
    }
    println!("deck is {:?}\n", my_deck);
    println!("table is {:?}\n", my_table);
    println!("hands are {:?}\n", my_hands);
}
