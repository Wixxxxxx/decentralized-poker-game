use crate::dealer::Dealer;
use crate::game_state::GameState;

pub struct PokerGame<'a> {
    state: &'a mut GameState,
    engine: &'a mut Dealer<'a>,
}

impl<'a> PokerGame<'a> {
    pub fn new(number_of_players: usize) -> Self {
        PokerGame {
            state: &mut GameState::new(number_of_players),
            engine: &mut Dealer::new(),
        }
    }
}
