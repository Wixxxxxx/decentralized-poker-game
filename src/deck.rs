enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

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
}

pub struct Card {
    suit: Suit,
    rank: Rank,
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() {
        // add code here to build the deck
    }
}
