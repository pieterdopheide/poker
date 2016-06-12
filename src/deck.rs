use card::{Card, Rank, Suit};
use rand::{thread_rng, Rng};

pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    #[inline]
    fn default() -> Deck {
        Deck { cards: vec![
            Card { rank: Rank::Ace, suit: Suit::Club }, Card { rank: Rank::Two, suit: Suit::Club }, Card { rank: Rank::Three, suit: Suit::Club }, Card { rank: Rank::Four, suit: Suit::Club }, Card { rank: Rank::Five, suit: Suit::Club }, Card { rank: Rank::Six, suit: Suit::Club }, Card { rank: Rank::Seven, suit: Suit::Club }, Card { rank: Rank::Eight, suit: Suit::Club }, Card { rank: Rank::Nine, suit: Suit::Club }, Card { rank: Rank::Ten, suit: Suit::Club }, Card { rank: Rank::Jack, suit: Suit::Club }, Card { rank: Rank::Queen, suit: Suit::Club }, Card { rank: Rank::King, suit: Suit::Club },
            Card { rank: Rank::Ace, suit: Suit::Diamond }, Card { rank: Rank::Two, suit: Suit::Diamond }, Card { rank: Rank::Three, suit: Suit::Diamond }, Card { rank: Rank::Four, suit: Suit::Diamond }, Card { rank: Rank::Five, suit: Suit::Diamond }, Card { rank: Rank::Six, suit: Suit::Diamond }, Card { rank: Rank::Seven, suit: Suit::Diamond }, Card { rank: Rank::Eight, suit: Suit::Diamond }, Card { rank: Rank::Nine, suit: Suit::Diamond }, Card { rank: Rank::Ten, suit: Suit::Diamond }, Card { rank: Rank::Jack, suit: Suit::Diamond }, Card { rank: Rank::Queen, suit: Suit::Diamond }, Card { rank: Rank::King, suit: Suit::Diamond },
            Card { rank: Rank::Ace, suit: Suit::Heart }, Card { rank: Rank::Two, suit: Suit::Heart }, Card { rank: Rank::Three, suit: Suit::Heart }, Card { rank: Rank::Four, suit: Suit::Heart }, Card { rank: Rank::Five, suit: Suit::Heart }, Card { rank: Rank::Six, suit: Suit::Heart }, Card { rank: Rank::Seven, suit: Suit::Heart }, Card { rank: Rank::Eight, suit: Suit::Heart }, Card { rank: Rank::Nine, suit: Suit::Heart }, Card { rank: Rank::Ten, suit: Suit::Heart }, Card { rank: Rank::Jack, suit: Suit::Heart }, Card { rank: Rank::Queen, suit: Suit::Heart }, Card { rank: Rank::King, suit: Suit::Heart },
            Card { rank: Rank::Ace, suit: Suit::Spade }, Card { rank: Rank::Two, suit: Suit::Spade }, Card { rank: Rank::Three, suit: Suit::Spade }, Card { rank: Rank::Four, suit: Suit::Spade }, Card { rank: Rank::Five, suit: Suit::Spade }, Card { rank: Rank::Six, suit: Suit::Spade }, Card { rank: Rank::Seven, suit: Suit::Spade }, Card { rank: Rank::Eight, suit: Suit::Spade }, Card { rank: Rank::Nine, suit: Suit::Spade }, Card { rank: Rank::Ten, suit: Suit::Spade }, Card { rank: Rank::Jack, suit: Suit::Spade }, Card { rank: Rank::Queen, suit: Suit::Spade }, Card { rank: Rank::King, suit: Suit::Spade },
        ] }
    }
}

impl Deck {
    pub fn shuffle_deck(&mut self) {
        // shuffle deck
        let slice: &mut [Card] = self.cards.as_mut_slice();
        thread_rng().shuffle(slice);
    }

    pub fn draw_card(&mut self) -> Card {
        // draw a card
        self.cards.pop().unwrap()
    }

    pub fn deck_size(&self) {
        // show cards remaining in the deck
        println!("{}", self.cards.len());
    }
}

