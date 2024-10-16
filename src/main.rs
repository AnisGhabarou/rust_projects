mod game;
mod input;
mod config;

use crate::game::GuessingGame;
use crate::input::{choose_game_mode, get_range_from_user, choose_difficulty};
use crate::config::GameConfig;

fn main() {
    loop {
        println!("Welcome to the Guessing Game!");

        // Let the user choose between Free Mode and Standard Mode
        let game_mode = choose_game_mode();

        let (lower_bound, upper_bound) = match game_mode.as_str() {
            "free" => get_range_from_user(),
            "standard" => choose_difficulty(),
            _ => unreachable!("Invalid game mode"),
        };
        
        // Configure the game
        let config = GameConfig::new(lower_bound, upper_bound);
        let mut game = GuessingGame::new(config);

        // Start the game
        game.start();

        // Ask if the user wants to play again
        println!("Do you want to play again? (yes/no)\nHint: any input other than 'yes' will automatically be considered as 'no'.");

        let play_again = input::get_user_input().trim().to_lowercase();
        if play_again != "yes" {
            println!("I hope you enjoyed it.\nThank you for playing!");
            break;
        }
    }
}