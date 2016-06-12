use card::Card;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Ranking {
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
    cards: Vec<Card>,
    ranking: Ranking,
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

    fn discard_card(&mut self, card: Card) {
        //self.cards.into_iter().filter(|&i|i.rank == card.rank && i.suit == card.suit).collect::<Vec<_>>();
        self.cards.pop();
    }

    pub fn show_hand(&self) {
        for card in &self.cards {
            card.print_card();
            print!(" ");
        }
    }

    fn print_ranking(&self) {
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

