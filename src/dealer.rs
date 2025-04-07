// this is the counterpart to the traditional game engine
// the dealer changes the game state by dealing cards, managing turns
// anything the changes the state of the game really

use crate::{deck::DeckError, game_state::GameState};

pub struct Dealer<'a> {
    state: &'a mut GameState,
}

impl<'a> Dealer<'a> {
    pub fn new(state: &'a mut GameState) -> Self {
        Dealer { state }
    }

    pub fn distribute_cards(&mut self) -> Result<(), DeckError> {
        let number_of_players = self.state.players.len();
        for dealt in 0..number_of_players {
            let player_idx = dealt % number_of_players;
            let card = self.state.deck.draw_card()?;
            self.state
                .players
                .get_mut(&player_idx)
                .unwrap()
                .hand
                .push(card);
        }

        Ok(())
    }

    pub fn deal_community_cards(&mut self, number_of_cards: usize) -> Result<(), DeckError> {
        let cards = self.state.deck.remove_from_top(number_of_cards)?;
        self.state.community_cards.extend(cards);
        Ok(())
    }

    pub fn showdown() {}
    pub fn take_action() {}
}
