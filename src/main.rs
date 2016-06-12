extern crate rand;

pub mod card;
pub mod deck;

use deck::Deck;

fn draw_hand(mut deck: Deck) -> Deck {
    for x in 0..5 {
        deck.draw_card().print_card();
        if x != 4 {
            print!(", ");
        }
    }
    deck
}

fn main() {
    let mut deck = Deck::default();

    deck.shuffle_deck();

    println!("First hand:");
    let deck = draw_hand(deck);

    println!("Second hand:");
    draw_hand(deck);
}

