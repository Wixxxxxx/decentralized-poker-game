use crate::dealer::Dealer;
use crate::game_state::GameState;

pub struct PokerGame {
    state: GameState,
    engine: Dealer,
}

impl PokerGame {
    pub fn new(number_of_players: usize) -> Self {
        let mut state = GameState::new(number_of_players);
        PokerGame {
            state,
            engine: Dealer::new(),
        }
    }

    pub fn advance_round() {}
    pub fn run() {}
}
