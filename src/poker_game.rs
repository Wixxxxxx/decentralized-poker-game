use crate::dealer::Dealer;
use crate::game_state::GameState;

pub struct PokerGame {
    state: GameState,
    engine: Dealer,
}

impl PokerGame {
    // initializes the game
    pub fn new(number_of_players: usize, buy_in: u8) -> Self {
        let state = GameState::new(number_of_players, buy_in);

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
