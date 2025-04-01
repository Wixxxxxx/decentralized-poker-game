use rand::rng;
use rand::seq::SliceRandom;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeckError {
    #[error("The deck is empty!")]
    EmptyDeck,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: vec![
                // init hearts
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Two,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Three,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Four,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Five,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Six,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Seven,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Eight,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Nine,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Ten,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Jack,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Queen,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::King,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Ace,
                },
                // init diamonds
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Two,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Three,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Four,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Five,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Six,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Seven,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Eight,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Nine,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ten,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Jack,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Queen,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::King,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace,
                },
                // init clubs
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Three,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Four,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Five,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Seven,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Eight,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Nine,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ten,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Jack,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Queen,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::King,
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace,
                },
                // init spades
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Two,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Three,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Four,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Five,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Six,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Seven,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Eight,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Nine,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Ten,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Jack,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Queen,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::King,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                },
            ],
        }
    }

    pub fn shuffle_deck(&mut self) {
        // uses Fisher-Yates shuffle
        self.cards.shuffle(&mut rng());
    }

    pub fn draw_card(&mut self) -> Result<Card, DeckError> {
        self.cards.pop().ok_or_else(|| DeckError::EmptyDeck)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
