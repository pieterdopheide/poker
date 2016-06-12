mod card;

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
}

struct Hand {
    cards: Vec<Card>,
    ranking: Ranking,
}

pub impl Hand {
    fn add_card(&self, card: Card) {
        self.cards.push(card);
    }

    fn remove_card(&self, card: Card) {
        self.cards.into_iter().filter(|&i|i.rank == card.rank && i.suit == card.suit).collect::<Vec<_>>();
        println!("{:?}", self.cards);
    }

    fn show_hand(&self) {
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
        }
    }
}

