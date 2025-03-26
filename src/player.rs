use crate::deck::Card;

pub struct Player {
    pub name: Option<String>,
    hand: Option<Vec<Card>>,
    stack: f64,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: Some(String::from(name)),
            hand: None,
            stack: 0.0,
        }
    }
}

// implement all poker actions that a player can take
// make sure to update gamestate accordingly
//
