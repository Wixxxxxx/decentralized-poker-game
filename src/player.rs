use crate::deck::Card;
use crate::game_state::GameState;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("Player must match current bet to check")]
    CheckError,
}

#[derive(PartialEq, Eq, Hash)]
pub struct PlayerId(pub usize);

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

    pub fn fold(&mut self) {
        self.has_folded = true;
    }

    pub fn check(&self, state: &GameState) {
        if self.current_bet == state.current_bet {
            Ok(())
        } else {
            Err("")
        }
    }
}

// helper function to initialize map of players to be tracked in game state
pub fn create_player_map(number_of_players: usize) -> HashMap<PlayerId, Player> {
    let mut players: HashMap<PlayerId, Player> = HashMap::new();
    for num in 0..number_of_players {
        players.insert(PlayerId(num), Player::new(format!("Player {}", num)));
    }
    players
}
