use std::collections::HashMap;

use card::{Card, Rank, Suit};
use hand::Ranking;

fn prepare(cards: Vec<Card>) -> usize {
    let mut hand_map: HashMap<Rank, i32> = HashMap::new();

    for card in cards {
        let card_rank = card.rank;

        if !hand_map.contains_key(&card_rank) {
            hand_map.insert(card_rank, 1);
        }
    }

    hand_map.len()
}

fn count_suits(cards: Vec<Card>) -> Vec<i32> {
    let mut suit_count = vec![0, 0, 0, 0];

    for card in cards {
        match card.suit {
            Suit::Club => suit_count[0] += 1,
            Suit::Diamond => suit_count[1] += 1,
            Suit::Heart => suit_count[2] += 1,
            Suit::Spade => suit_count[3] += 1,
        }
    }

    suit_count
}

fn full_house_or_four_of_a_kind(cards: Vec<Card>) -> Ranking {
    let first_rank = cards[0].rank;
    let mut rank_count = 0;

    for card in cards {
        if first_rank == card.rank {
            rank_count += 1;
        }
    }

    if rank_count == 4 || rank_count == 1 {
        return Ranking::FourOfAKind;
    } else {
        return Ranking::FullHouse;
    }
}

fn two_pair_or_three_of_a_kind(cards: Vec<Card>) -> Ranking {
    let suit_count = count_suits(cards);

    for x in 0..suit_count.len() {
        if suit_count[x] == 0 {
            for y in x+1..suit_count.len() {
                if suit_count[y] == 0 {
                    return Ranking::ThreeOfAKind;
                }
            }
        }
    }

    Ranking::TwoPair
}

fn further_evaluation(mut cards: Vec<Card>) -> Ranking {
    let previous_suit: Suit;
    let mut is_flush = false;
    let mut is_straight = false;

    if cards[0].suit == cards[1].suit
        && cards[1].suit == cards[2].suit
        && cards[2].suit == cards[3].suit
        && cards[3].suit == cards[4].suit {
            is_flush = true;
    }

    let ranks = vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

    let mut valued_ranks: Vec<usize> = Vec::new();

    for x in 0..cards.len() {
        let temp_index = x + 1;

        for y in temp_index..cards.len() {
            for z in 0..ranks.len() {
                let mut cardx_rank = 0;
                let mut cardy_rank = 0;

                if ranks[z] == cards[x].rank {
                    cardx_rank = z;
                }
                if ranks[z] == cards[y].rank {
                    cardy_rank = z;
                }
                if cardx_rank < cardy_rank {
                    let temp_value = cards[y];
                    cards[y] = cards[x];
                    cards[x] = temp_value;
                    valued_ranks.push(cardx_rank);
                }
            }
        }
    }

    if valued_ranks[0] == valued_ranks[1] + 1
        && valued_ranks[1] == valued_ranks[2] + 1
        && valued_ranks[2] == valued_ranks[3] + 1
        && valued_ranks[3] == valued_ranks[4] + 1 {
            is_straight = true;
    }

    if is_flush && is_straight {
        return Ranking::StraightFlush;
    } else if is_flush {
        return Ranking::Flush;
    } else if is_straight {
        return Ranking::Straight;
    }

    Ranking::HighCard
}

pub fn rank_hand(cards: Vec<Card>) -> Ranking {
    let hand_map_size = prepare(cards.clone());

    match hand_map_size {
        2 => full_house_or_four_of_a_kind(cards),
        3 => two_pair_or_three_of_a_kind(cards),
        4 => return Ranking::OnePair,
        5 => further_evaluation(cards),
        _ => return Ranking::None,
    }
}

