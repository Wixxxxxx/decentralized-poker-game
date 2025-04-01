use crate::betting_round::BettingRound;
use crate::deck::{Card, Deck};
use crate::player::Player;
use std::collections::HashMap;

pub struct GameState {
    players: HashMap<u8, Player>,
    pot: f64,
    betting_round: BettingRound,
    small_blind: u8,
    big_blind: u8,
    deck: Deck,
    most_recent_bet: u8,
    pub community_cards: Vec<Card>,
}

impl GameState {
    pub fn new() {}
    pub fn add_flop() {}
    pub fn add_turn() {}
    pub fn add_river() {}
    pub fn showdown() {}
    pub fn take_action() {}
    pub fn distribute_cards() {}
}
