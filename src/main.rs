extern crate rand;

pub mod card;
pub mod deck;
pub mod hand;

use deck::Deck;
use hand::Hand;

fn draw_hand(mut deck: Deck, mut hand: Hand) -> (Deck, Hand) {
    for x in 0..5 {
        hand.add_card(deck.draw_card());
    }
    (deck, hand)
}

fn main() {
    let mut deck = Deck::default();
    let hand1 = Hand::default();
    let hand2 = Hand::default();

    deck.shuffle_deck();

    let (deck, hand1) = draw_hand(deck, hand1);
    let (deck, hand2) = draw_hand(deck, hand2);

    println!("First hand:");
    hand1.show_hand();

    println!("\nSecond hand:");
    hand2.show_hand();
}

