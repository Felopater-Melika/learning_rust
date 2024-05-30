mod programs;

use crate::programs::calculator::calculator;
use crate::programs::guessing_game::guessing_game;
use crate::programs::mini_grep::mini_grep;
use learning_rust::read_input;

fn main() {
    let mut prompt = String::from("Please select a program to run:\n");
    let programs: [&str; 3] = ["Calculator", "Guessing game", "Mini grep"];

    for (index, program) in programs.iter().enumerate() {
        prompt.push_str(&format!("{}. {}\n", index + 1, program));
    }

    let program_number = match read_input(prompt.as_str()) {
        Ok(input) => input.trim().parse::<i8>().unwrap(),
        Err(e) => {
            eprintln!("Error reading program number: {}", e);
            return;
        }
    };

    match program_number {
        1 => calculator(),
        2 => guessing_game(),
        3 => mini_grep(),
        _ => println!("Invalid program number"),
    }
}
