use crate::betting_round::BettingRound;
use crate::deck::{Card, Deck};
use crate::player::{create_player_map, Player};
use std::collections::HashMap;

pub struct GameState {
    pub players: HashMap<usize, Player>,
    pub deck: Deck,
    pot: u8,
    betting_round: BettingRound,
    small_blind: u8,
    big_blind: u8,
    most_recent_bet: u8,
    pub community_cards: Vec<Card>,
}

impl GameState {
    pub fn new(number_of_players: usize) -> Self {
        GameState {
            players: create_player_map(number_of_players),
            deck: Deck::new(),
            pot: 0,
            betting_round: BettingRound::PreFlop,
            small_blind: 0,
            big_blind: 1,
            most_recent_bet: 0,
            community_cards: Vec::new(),
        }
    }
}
