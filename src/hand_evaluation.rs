use crate::{
    deck::{Card, Rank, Suit},
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
    sorted_ranks: Vec<Rank>,
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
        let sorted_ranks = player
            .hand
            .iter()
            .map(|card| card.rank)
            .sorted()
            .dedup()
            .collect::<Vec<Rank>>();

        HandAnalysis {
            rank_counts,
            suit_counts,
            sorted_ranks,
        }
    }

    pub fn evaluate_hand(&self) -> HandRank {
        let mut rank_counts_lst: Vec<usize> = self.rank_counts.values().copied().collect();
        rank_counts_lst.sort_unstable_by(|r1, r2| r2.cmp(r1));

        let mut suit_counts_lst: Vec<usize> = self.suit_counts.values().copied().collect();
        suit_counts_lst.sort_unstable_by(|r1, r2| r2.cmp(r1));

        let is_flush = self.suit_counts.values().any(|&count| count >= 5);
        let is_straight = is_straight(&self.sorted_ranks);

        match rank_counts_lst.as_slice() {
            [4, ..] => HandRank::FourOfAKind,
            [3, 2, ..] => HandRank::FullHouse,
            [3, ..] => HandRank::ThreeOfAKind,
            [2, 2, ..] => HandRank::TwoPair,
            [2, ..] => HandRank::Pair,
            _ => HandRank::HighCard,
        }
    }
}

pub fn is_straight(ranks: &[Rank]) -> bool {
    // case: Ace as low card
    if ranks == [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace] {
        return true;
    }

    ranks.windows(5).any(|window| {
        window
            .iter()
            .tuple_windows()
            .all(|(a, b)| b.value() == a.value() + 1)
    })
}
