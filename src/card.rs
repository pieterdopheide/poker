#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    fn print_rank(&self) {
        match self.rank {
            Rank::Two => print!("2"),
            Rank::Three => print!("3"),
            Rank::Four => print!("4"),
            Rank::Five => print!("5"),
            Rank::Six => print!("6"),
            Rank::Seven => print!("7"),
            Rank::Eight => print!("8"),
            Rank::Nine => print!("9"),
            Rank::Ten => print!("10"),
            Rank::Jack => print!("J"),
            Rank::Queen => print!("Q"),
            Rank::King => print!("K"),
            Rank::Ace => print!("A"),
        }
    }

    fn print_suit(&self) {
        match self.suit {
            Suit::Club => print!("C"),
            Suit::Diamond => print!("D"),
            Suit::Heart => print!("H"),
            Suit::Spade => print!("S"),
        }
    }

    pub fn print_card(&self) {
        &self.print_rank();
        &self.print_suit();
    }
}

