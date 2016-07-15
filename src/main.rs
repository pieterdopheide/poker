extern crate rand;

mod card;
mod deck;
mod hand;
mod hand_ranker;

use deck::Deck;
use hand::{Hand, Ranking};

fn draw_hand(deck: &mut Deck, mut hand: Hand) -> Hand {
    for x in 0..5 {
        hand.add_card(deck.draw_card());
    }
    hand
}

fn determine_winner(ranking1: Ranking, ranking2: Ranking) {
    let rankings = [Ranking::HighCard, Ranking::OnePair, Ranking::TwoPair, Ranking::ThreeOfAKind, Ranking::Straight, Ranking::Flush, Ranking::FullHouse, Ranking::FourOfAKind, Ranking::StraightFlush];

    let mut rank_value1 = 0;
    let mut rank_value2 = 0;

    for x in 0..rankings.len() {
        if rankings[x] == ranking1 {
            rank_value1 = x;
        }
        if rankings[x] == ranking2 {
            rank_value2 = x;
        }
    }

    if rank_value1 > rank_value2 {
        println!("Hand 1 wins!");
    } else if rank_value2 > rank_value1 {
        println!("Hand 2 wins!");
    } else {
        println!("It's a tie!");
    }
}

fn main() {
    let mut deck = Deck::default();
    let hand1 = Hand::default();
    let hand2 = Hand::default();

    deck.shuffle_deck();

    let mut hand1 = draw_hand(&mut deck, hand1);
    let mut hand2 = draw_hand(&mut deck, hand2);

    let ranking1 = hand_ranker::rank_hand(hand1.cards.clone());
    hand1.set_ranking(ranking1);
    let ranking2 = hand_ranker::rank_hand(hand2.cards.clone());
    hand2.set_ranking(ranking2);

    println!("First hand:");
    hand1.show_hand();
    hand1.print_ranking();

    println!("\nSecond hand:");
    hand2.show_hand();
    hand2.print_ranking();

    determine_winner(hand1.ranking, hand2.ranking);
}

