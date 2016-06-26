use card::Card;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Ranking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    None,
}

pub struct Hand {
    pub cards: Vec<Card>,
    pub ranking: Ranking,
}

impl Default for Hand {
    #[inline]
    fn default() -> Hand {
        Hand { cards: Vec::new(), ranking: Ranking::None }
    }
}

impl Hand {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn show_hand(&self) {
        for card in &self.cards {
            card.print_card();
            print!(" ");
        }
    }

    pub fn set_ranking(&mut self, ranking: Ranking) {
        self.ranking = ranking;
    }

    pub fn print_ranking(&self) {
        match self.ranking {
            Ranking::HighCard => println!("High card"),
            Ranking::OnePair => println!("One pair"),
            Ranking::TwoPair => println!("Two pair"),
            Ranking::ThreeOfAKind => println!("Three of a kind"),
            Ranking::Straight => println!("Straight"),
            Ranking::Flush => println!("Flush"),
            Ranking::FullHouse => println!("Full house"),
            Ranking::FourOfAKind => println!("Four of a kind"),
            Ranking::StraightFlush => println!("straight flush"),
            Ranking::None => println!("None"),
        }
    }
}

