use crate::{
    deck::{Rank, Suit},
    game_state::GameState,
    player::Player,
};
use itertools::Itertools;
use std::collections::HashMap;

// TODO: Add high cards and kickers for tie breakers!!!!
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
            .chain(game_state.community_cards.iter())
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

        let is_straight = is_straight(&self.sorted_ranks);

        match rank_counts_lst.as_slice() {
            [4, ..] => HandRank::FourOfAKind,
            [3, 2, ..] => HandRank::FullHouse,
            [3, ..] => HandRank::ThreeOfAKind,
            [2, 2, ..] => HandRank::TwoPair,
            [2, ..] => HandRank::Pair,
            _ => {
                if is_flush {
                    HandRank::Flush
                } else if is_straight {
                    HandRank::Straight
                } else {
                    HandRank::HighCard
                }
            }
        }
    }
}

// helper function to check for a straight
pub fn is_straight(ranks: &[Rank]) -> bool {
    if ranks.len() < 5 {
        return false;
    }

    // case: Ace as low card
    let low_ace = [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];
    if low_ace.iter().all(|r| ranks.contains(r)) {
        return true;
    }

    // case: any straight
    ranks.windows(5).any(|window| {
        window
            .iter()
            .tuple_windows()
            .all(|(a, b)| b.value() == a.value() + 1)
    })
}

// TODO: Implement flush function, return if flush, straight flush or neither
// pub fn flush_cards() -> HandRan {
//     let is_flush = self.suit_counts.values().any(|&count| count >= 5);

//     let is_royal = [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace]
//         .iter()
//         .all(|r| self.sorted_ranks.contains(r));

//     if is_flush && is_royal {
//         return HandRank::RoyalFlush;
//     }

//     if is_flush && is_straight {
//         return HandRank::StraightFlush;
//     }
// }
