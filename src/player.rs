use crate::deck::Card;

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    has_folded: bool,
    current_bet: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            hand: Vec::new(),
            has_folded: false,
            current_bet: 0,
        }
    }
}
