use decentralized_poker_game::deck::Deck;
use std::env;
use std::error::Error;

// use ratatui crate for terminal UI

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let number_of_players = &args[1].parse::<u8>()?;

    // init deck for game start
    let mut deck = Deck::new();
    deck.shuffle_deck();

    // add players and distribute hands
    // initialize game state --> randomly choose small and big blinds and then rotate after each hand

    // STRUCTURE
    // 1. round of betting
    // 2. add flop
    // 3. round of betting
    // 4. add turn
    // 5. round of betting
    // 6. add river
    // 7. round of betting
    // 8. showdown
    // 9. repeat
    Ok(())
}

// fn main() {
//     // to take user input
//     let mut player_number = String::new();
//     let mut buy_in = String::new();

//     println!("How many players?");

//     io::stdin()
//         .read_line(&mut player_number)
//         .expect("Couldn't read line");

//     println!("Buy in?");

//     io::stdin()
//         .read_line(&mut buy_in)
//         .expect("Couldn't read line");

//     let player_number: u8 = player_number.trim().parse().unwrap();
//     let buy_in: f64 = buy_in.trim().parse().unwrap();

//     // Initialize the GameState
//     let gs = GameState::new(player_number, buy_in);
// }
