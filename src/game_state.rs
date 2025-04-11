use crate::betting_round::BettingRound;
use crate::deck::Card;
use crate::player::{create_player_map, Player, PlayerId};
use std::collections::HashMap;

pub struct GameState {
    pub players: HashMap<PlayerId, Player>,
    pub play_order: Vec<PlayerId>,
    pot: u8,
    betting_round: BettingRound,
    small_blind: PlayerId,
    big_blind: PlayerId,
    most_recent_bet: u8,
    pub community_cards: Vec<Card>,
}

impl GameState {
    pub fn new(number_of_players: usize) -> Self {
        GameState {
            players: create_player_map(number_of_players),
            play_order: (0..number_of_players)
                .map(|x| PlayerId(x))
                .collect::<Vec<PlayerId>>(),
            pot: 0,
            betting_round: BettingRound::PreFlop,
            small_blind: PlayerId(0),
            big_blind: PlayerId(1),
            most_recent_bet: 0,
            community_cards: Vec::new(),
        }
    }
}
