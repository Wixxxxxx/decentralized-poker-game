use crate::player::Player;
use std::collections::HashMap;

struct GameState {
    players: HashMap<u8, Player>,
    pot: f64,
    small_blind: u8,
    big_blind: u8,
}

impl GameState {
    pub fn new(num_players: u8, buy_in: f64) -> Self {
        let mut players: HashMap<u8, Player> = HashMap::new();
        players.extend((0..num_players).map(|n| (n, Player::new("Test Name"))));

        GameState {
            players,
            pot: buy_in * (num_players as f64),
            small_blind: 0,
            big_blind: 1,
        }
    }
}
