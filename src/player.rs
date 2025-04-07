use std::collections::HashMap;

use crate::deck::Card;

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub has_folded: bool,
    pub current_bet: u32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            hand: Vec::new(),
            has_folded: false,
            current_bet: 0,
        }
    }
}

// helper function to initialize map of players to be tracked in game state
pub fn create_player_map(number_of_players: usize) -> HashMap<usize, Player> {
    let mut players: HashMap<usize, Player> = HashMap::new();
    for num in 0..number_of_players {
        players.insert(num, Player::new(format!("Player {}", num)));
    }
    players
}
