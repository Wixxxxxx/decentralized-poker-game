use decentralized_poker_game::dealer::Dealer;
use decentralized_poker_game::game_state::GameState;
use decentralized_poker_game::poker_game::PokerGame;
use std::env;
use std::error::Error;

// use ratatui crate for terminal UI

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // get number of players and predetermined buy-in
    let number_of_players = args[1].parse::<usize>()?;
    let ante = args[2].parse::<u8>()?;

    // init Game
    let poker_game = PokerGame::new();

    Ok(())
}
