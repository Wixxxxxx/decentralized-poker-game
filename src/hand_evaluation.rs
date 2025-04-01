use crate::{
    deck::{Rank, Suit},
    game_state::GameState,
    player::Player,
};
use itertools::Itertools;
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
    rank_counts: HashMap<Rank, usize>,
    suit_counts: HashMap<Suit, usize>,
}

impl HandAnalysis {
    pub fn new(player: &Player, game_state: &GameState) -> Self {
        let rank_counts = player
            .hand
            .iter()
            .chain(game_state.community_cards.iter())
            .counts_by(|card| card.rank);
        let suit_counts = player
            .hand
            .iter()
            .chain(game_state.community_cards.iter())
            .counts_by(|card| card.suit);

        HandAnalysis {
            rank_counts,
            suit_counts,
        }
    }

    pub fn determine_hand_rank(&self) -> HandRank {
        let mut ranks: Vec<usize> = self.rank_counts.values().copied().collect();
        ranks.sort_unstable_by(|r1, r2| r2.cmp(r1));
        let is_flush = self.suit_counts.values().any(|&count| count >= 5);

        match ranks.as_slice() {
            [4, ..] => HandRank::FourOfAKind,
            [3, 2, ..] => HandRank::FullHouse,
            _ => HandRank::RoyalFlush,
        }
    }
}
