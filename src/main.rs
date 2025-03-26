use poker_game::player::Player;
use poker_game::GameState;
use std::io;

fn main() {
    // to take user input
    let mut player_number = String::new();
    let mut buy_in = String::new();

    println!("How many players?");

    io::stdin()
        .read_line(&mut player_number)
        .expect("Couldn't read line");

    println!("Buy in?");

    io::stdin()
        .read_line(&mut buy_in)
        .expect("Couldn't read line");

    let player_number: u8 = player_number.trim().parse().unwrap();
    let buy_in: f64 = buy_in.trim().parse().unwrap();

    // Initialize the GameState
    let gs = GameState::new(player_number, buy_in);
}
