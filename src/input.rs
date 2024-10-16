use std::io;
pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string() // Trim whitespace and return as String
}

pub fn get_input_as<T: std::str::FromStr>() -> T {
    loop {
        let input = get_user_input();
        match input.parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid input."),
        }
    }
}

pub fn get_range_from_user() -> (u32, u32) {
    println!("Enter the lower bound:");
    let lower_bound: u32 = get_input_as();

    println!("Enter the upper bound:");
    let upper_bound: u32 = get_input_as();

    if lower_bound >= upper_bound {
        println!("Error: The lower bound must be less than the upper bound.");
        return get_range_from_user(); // Recursive call if bounds are invalid
    }

    (lower_bound, upper_bound)
}

pub fn choose_difficulty() -> (u32, u32) {
    println!("Choose a difficulty level (easy, medium, hard):");

    loop {
        let difficulty = get_user_input();

        match difficulty.as_str() {
            "easy" => return (1, 50),
            "medium" => return (1, 100),
            "hard" => return (1, 500),
            _ => println!("Please choose a valid difficulty: easy, medium, or hard."),
        }
    }
}

pub fn choose_game_mode() -> String {
    println!("Choose the game mode (free/standard):");

    loop {
        let mode = get_user_input();

        match mode.as_str() {
            "free" => return "free".to_string(),
            "standard" => return "standard".to_string(),
            _ => println!("Please choose a valid mode: free or standard."),
        }
    }
}





