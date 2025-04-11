// this is the counterpart to the traditional game engine
// the dealer changes the game state by dealing cards, managing turns
// anything the changes the state of the game really

use crate::{
    deck::{Deck, DeckError},
    game_state::GameState,
};

pub struct Dealer {
    pub deck: Deck,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { deck: Deck::new() }
    }

    pub fn distribute_cards(&mut self, state: &mut GameState) -> Result<(), DeckError> {
        let number_of_players = state.play_order.len();
        for dealt in 0..number_of_players {
            let player_idx = dealt % number_of_players;
            let card = self.deck.draw_card()?;
            state
                .players
                .get_mut(&state.play_order[player_idx])
                .unwrap()
                .hand
                .push(card);
        }

        Ok(())
    }

    pub fn deal_community_cards(
        &mut self,
        state: &mut GameState,
        number_of_cards: usize,
    ) -> Result<(), DeckError> {
        let cards = self.deck.remove_from_top(number_of_cards)?;
        state.community_cards.extend(cards);
        Ok(())
    }

    pub fn showdown() {}
}
