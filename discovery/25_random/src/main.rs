use rand::seq::SliceRandom;
use rand::{random_bool, random_range, rng};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
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
    Joker,
}

#[derive(Debug)]
#[allow(unused)]
struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        for suit in suits {
            for rank in ranks {
                cards.push(Card {
                    rank,
                    suit: Some(suit),
                });
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut my_rng = rng();
        self.cards.shuffle(&mut my_rng);
    }

    fn insert_jokers(&mut self) {
        for _ in 0..2 {
            self.cards.insert(
                random_range(0..52),
                Card {
                    rank: Rank::Joker,
                    suit: None,
                },
            );
        }
    }

    fn delete_random_card(&mut self) {
        // deletes a card 65% of the time
        if random_bool(0.65) {
            self.cards.remove(random_range(0..52));
        }
    }
}

fn main() {
    // creates ordered deck
    let mut deck = Deck::new();
    println!("{:?}", deck);

    // shuffles deck
    deck.shuffle();
    println!("{:?}", deck);

    deck.insert_jokers();
    println!("{:?}", deck);

    for _ in 0..10 {
        deck.delete_random_card();
    }

    println!("{}", deck.cards.len())
}
