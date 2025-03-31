use crate::{deck::{Rank, Suit}, player::Player, game_state::GameState};
use std::collections::HashMap;

pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub struct HandAnalysis {
    rank_counts: HashMap<Rank, u8>,
    suit_counts: HashMap<Suit, u8>,
}

impl HandAnalysis {
    pub fn new(player: &Player, game_state: &GameState) --> Self {
        let rank_counts = player.hand.fold()
        let suit_counts = HashMap::new();


    }
}
