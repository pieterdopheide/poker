use std::collections::HashMap;

use card::{Card, Rank, Suit};
use hand::Ranking;

fn prepare(cards: &Vec<Card>) -> usize {
    let mut hand_map: HashMap<Rank, i32> = HashMap::new();

    for card in cards {
        let card_rank = card.rank;

        if !hand_map.contains_key(&card_rank) {
            hand_map.insert(card_rank, 1);
        }
    }

    hand_map.len()
}

fn sort_cards_desc(cards: &mut Vec<Card>) {
    let ranks = vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

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
                }
            }
        }
    }
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

fn two_pair_or_three_of_a_kind(mut cards: Vec<Card>) -> Ranking {
    sort_cards_desc(&mut cards);

    for x in 0..cards.len() {
        if x + 2 <= cards.len()
            && cards[x].rank == cards[x + 1].rank
            && cards[x + 1].rank == cards[x + 2].rank {
                return Ranking::ThreeOfAKind;
        }
    }

    Ranking::TwoPair
}

fn further_evaluation(mut cards: Vec<Card>) -> Ranking {
    let mut is_flush = false;
    let mut is_straight = false;

    if cards[0].suit == cards[1].suit
        && cards[1].suit == cards[2].suit
        && cards[2].suit == cards[3].suit
        && cards[3].suit == cards[4].suit {
            is_flush = true;
    }

    sort_cards_desc(&mut cards);
    let mut valued_ranks: Vec<usize> = Vec::new();
    let ranks = vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

    for card in cards {
        for z in 0..ranks.len() {
            if ranks[z] == card.rank {
                valued_ranks.push(z);
            }
        }
    }

    if (valued_ranks[0] == valued_ranks[1] + 1
        || valued_ranks[0] == 12)
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
    let hand_map_size = prepare(&cards);

    match hand_map_size {
        2 => full_house_or_four_of_a_kind(cards),
        3 => two_pair_or_three_of_a_kind(cards),
        4 => return Ranking::OnePair,
        5 => further_evaluation(cards),
        _ => return Ranking::None,
    }
}

#[test]
fn rank_hand_straightflush() {
    let test_hand_straightflush = vec![Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Club }, Card { rank: Rank::Nine, suit: Suit::Club }, Card { rank: Rank::Eight, suit: Suit::Club }, Card { rank: Rank::Seven, suit: Suit::Club }];

    assert_eq!(Ranking::StraightFlush, rank_hand(test_hand_straightflush));
}

#[test]
fn rank_hand_fourofakind() {
    let test_hand_fourofakind = vec![Card { rank: Rank::Six, suit: Suit::Club }, Card { rank: Rank::Six, suit: Suit::Diamond }, Card { rank: Rank::Six, suit: Suit::Heart }, Card { rank: Rank::Six, suit: Suit::Spade }, Card { rank: Rank::Jack, suit: Suit::Heart }];

    assert_eq!(Ranking::FourOfAKind, rank_hand(test_hand_fourofakind));
}

#[test]
fn rank_hand_fullhouse() {
    let test_hand_fullhouse = vec![Card { rank: Rank::Five, suit: Suit::Spade }, Card { rank: Rank::Five, suit: Suit::Club }, Card { rank: Rank::Five, suit: Suit::Diamond }, Card { rank: Rank::King, suit: Suit::Club }, Card { rank: Rank::King, suit: Suit::Heart }];

    assert_eq!(Ranking::FullHouse, rank_hand(test_hand_fullhouse));
}

#[test]
fn rank_hand_flush() {
    let test_hand_flush = vec![Card { rank: Rank::Jack, suit: Suit::Diamond }, Card { rank: Rank::Ten, suit: Suit::Diamond }, Card { rank: Rank::Eight, suit: Suit::Diamond }, Card { rank: Rank::Seven, suit: Suit::Diamond }, Card { rank: Rank::Two, suit: Suit::Diamond }];

    assert_eq!(Ranking::Flush, rank_hand(test_hand_flush));
}

#[test]
fn rank_hand_straight() {
    let test_hand_straight = vec![Card { rank: Rank::King, suit: Suit::Diamond }, Card { rank: Rank::Queen, suit: Suit::Spade }, Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Heart }, Card { rank: Rank::Nine, suit: Suit::Spade }];

    assert_eq!(Ranking::Straight, rank_hand(test_hand_straight));
}

#[test]
fn rank_hand_low_ace_straight() {
    let test_hand_low_ace_straight = vec![Card { rank: Rank::Ace, suit: Suit::Diamond },
                                            Card { rank: Rank::Two, suit: Suit::Spade },
                                            Card { rank: Rank::Three, suit: Suit::Club },
                                            Card { rank: Rank::Four, suit: Suit::Heart },
                                            Card { rank: Rank::Five, suit: Suit::Spade }];
    assert_eq!(Ranking::Straight, rank_hand(test_hand_low_ace_straight));
}

#[test]
fn rank_hand_threeofakind() {
    let test_hand_threeofakind = vec![Card { rank: Rank::Ace, suit: Suit::Club }, Card { rank: Rank::Ace, suit: Suit::Spade }, Card { rank: Rank::Ace, suit: Suit::Heart }, Card { rank: Rank::Queen, suit: Suit::Diamond }, Card { rank: Rank::Two, suit: Suit::Spade }];

    assert_eq!(Ranking::ThreeOfAKind, rank_hand(test_hand_threeofakind));
}

#[test]
fn rank_hand_twopair() {
    let test_hand_twopair = vec![Card { rank: Rank::Queen, suit: Suit::Heart }, Card { rank: Rank::Queen, suit: Suit::Spade }, Card { rank: Rank::Eight, suit: Suit::Heart }, Card { rank: Rank::Eight, suit: Suit::Spade }, Card { rank: Rank::Two, suit: Suit::Club }];

    assert_eq!(Ranking::TwoPair, rank_hand(test_hand_twopair));
}

#[test]
fn rank_hand_onepair() {
    let test_hand_onepair = vec![Card { rank: Rank::Seven, suit: Suit::Spade }, Card { rank: Rank::Seven, suit: Suit::Heart }, Card { rank: Rank::King, suit: Suit::Spade }, Card { rank: Rank::Four, suit: Suit::Club }, Card { rank: Rank::Three, suit: Suit::Spade }];

    assert_eq!(Ranking::OnePair, rank_hand(test_hand_onepair));
}

#[test]
fn rank_hand_highcard() {
    let test_hand_highcard = vec![Card { rank: Rank::Ace, suit: Suit::Diamond }, Card { rank: Rank::Ten, suit: Suit::Spade }, Card { rank: Rank::Nine, suit: Suit::Heart }, Card { rank: Rank::Four, suit: Suit::Diamond }, Card { rank: Rank::Three, suit: Suit::Club }];

    assert_eq!(Ranking::HighCard, rank_hand(test_hand_highcard));
}

#[test]
fn test_prepare() {
    let test_hand_straightflush = vec![Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Club }, Card { rank: Rank::Nine, suit: Suit::Club }, Card { rank: Rank::Eight, suit: Suit::Club }, Card { rank: Rank::Seven, suit: Suit::Club }];

    assert_eq!(5, prepare(&test_hand_straightflush));
}

#[test]
fn full_house() {
    let test_hand_fullhouse = vec![Card { rank: Rank::Five, suit: Suit::Spade }, Card { rank: Rank::Five, suit: Suit::Club }, Card { rank: Rank::Five, suit: Suit::Diamond }, Card { rank: Rank::King, suit: Suit::Club }, Card { rank: Rank::King, suit: Suit::Heart }];

    assert_eq!(Ranking::FullHouse, full_house_or_four_of_a_kind(test_hand_fullhouse));
}

#[test]
fn four_of_a_kind() {
    let test_hand_fourofakind = vec![Card { rank: Rank::Six, suit: Suit::Club }, Card { rank: Rank::Six, suit: Suit::Diamond }, Card { rank: Rank::Six, suit: Suit::Heart }, Card { rank: Rank::Six, suit: Suit::Spade }, Card { rank: Rank::Jack, suit: Suit::Heart }];

    assert_eq!(Ranking::FourOfAKind, full_house_or_four_of_a_kind(test_hand_fourofakind));
}

#[test]
fn two_pair() {
    let test_hand_twopair = vec![Card { rank: Rank::Queen, suit: Suit::Heart }, Card { rank: Rank::Queen, suit: Suit::Spade }, Card { rank: Rank::Eight, suit: Suit::Heart }, Card { rank: Rank::Eight, suit: Suit::Spade }, Card { rank: Rank::Two, suit: Suit::Club }];

    assert_eq!(Ranking::TwoPair, two_pair_or_three_of_a_kind(test_hand_twopair));
}

#[test]
fn three_of_a_kind() {
    let test_hand_threeofakind = vec![Card { rank: Rank::Ace, suit: Suit::Club }, Card { rank: Rank::Ace, suit: Suit::Spade }, Card { rank: Rank::Ace, suit: Suit::Heart }, Card { rank: Rank::Queen, suit: Suit::Diamond }, Card { rank: Rank::Two, suit: Suit::Spade }];

    assert_eq!(Ranking::ThreeOfAKind, two_pair_or_three_of_a_kind(test_hand_threeofakind));
}

#[test]
fn further_evaluation_straightflush() {
    let test_hand_straightflush = vec![Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Club }, Card { rank: Rank::Nine, suit: Suit::Club }, Card { rank: Rank::Eight, suit: Suit::Club }, Card { rank: Rank::Seven, suit: Suit::Club }];

    assert_eq!(Ranking::StraightFlush, further_evaluation(test_hand_straightflush));
}

#[test]
fn further_evaluation_flush() {
    let test_hand_flush = vec![Card { rank: Rank::Jack, suit: Suit::Diamond }, Card { rank: Rank::Ten, suit: Suit::Diamond }, Card { rank: Rank::Eight, suit: Suit::Diamond }, Card { rank: Rank::Seven, suit: Suit::Diamond }, Card { rank: Rank::Two, suit: Suit::Diamond }];

    assert_eq!(Ranking::Flush, further_evaluation(test_hand_flush));
}

#[test]
fn further_evaluation_straight() {
    let test_hand_straight = vec![Card { rank: Rank::King, suit: Suit::Diamond }, Card { rank: Rank::Queen, suit: Suit::Spade }, Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Heart }, Card { rank: Rank::Nine, suit: Suit::Spade }];

    assert_eq!(Ranking::Straight, further_evaluation(test_hand_straight));
}

#[test]
fn further_evaluation_highcard() {
    let test_hand_highcard = vec![Card { rank: Rank::Ace, suit: Suit::Diamond }, Card { rank: Rank::Ten, suit: Suit::Spade }, Card { rank: Rank::Nine, suit: Suit::Heart }, Card { rank: Rank::Four, suit: Suit::Diamond }, Card { rank: Rank::Three, suit: Suit::Club }];

    assert_eq!(Ranking::HighCard, further_evaluation(test_hand_highcard));
}

