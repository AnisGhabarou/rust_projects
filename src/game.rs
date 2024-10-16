use rand::Rng;
use std::cmp::Ordering;
use crate::config::GameConfig;
use crate::input::get_input_as;
use colored::*;

pub struct GuessingGame {
    secret_number: u32,
    attempts: u32,
    config: GameConfig,
}

impl GuessingGame {
    pub fn new(config: GameConfig) -> Self {
        let secret_number = rand::thread_rng().gen_range(config.lower_bound..=config.upper_bound);
        Self {
            secret_number,
            attempts: 0,
            config,
        }
    }

    pub fn start(&mut self) {
        println!(
            "Guess the number between {} and {}!",
            self.config.lower_bound, self.config.upper_bound
        );

        loop {
            self.attempts += 1;

            println!("Please input your guess:");
            let guess: u32 = get_input_as();

            match guess.cmp(&self.secret_number) {
                Ordering::Less => println!("{}", "Too small!".red()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    self.display_result();
                    break;
                }
            }

            /*if self.attempts >= 3 {
                println!("You win on the {}-th try!", self.attempts);
            }*/
        }
        /*if self.attempts >= 3 {
            println!("You win on the {}-th try!", self.attempts);
        }*/
    }
    
    fn display_result(&self) {
        match self.attempts {
            1 => println!("{}", "Excellent! You guessed the number on the first try!".green()),
            2 => println!("{}", "Very good! You guessed the number on the second try!".green()),
            3 => println!("{}", "Good! You guessed the number on the third try!".green()),
            _ => println!("You win on the {}-th try!", self.attempts),
        }
    }
}


