use crate::dealer::Dealer;
use crate::game_state::GameState;

pub struct PokerGame {
    state: GameState,
    engine: Dealer,
}

impl PokerGame {
    // initializes the game
    pub fn new(number_of_players: usize) -> Self {
        let state = GameState::new(number_of_players);

        PokerGame {
            state,
            engine: Dealer::new(),
        }
    }

    // advances the game phase
    pub fn advance_round() {}

    // manages the game loop
    pub fn run() {}
}
