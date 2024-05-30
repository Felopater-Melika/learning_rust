use learning_rust::read_input;
use rand::Rng;

pub fn guessing_game() {
    println!("Guess the number from 1 to 10!");
    let secret_number: i8 = rand::thread_rng().gen_range(1..11);
    loop {
        match read_input("Enter your guess:") {
            Ok(guess) => match guess.trim().parse::<i8>() {
                Ok(guess) => match guess.cmp(&secret_number) {
                    std::cmp::Ordering::Less => println!("Too small!"),
                    std::cmp::Ordering::Greater => println!("Too big!"),
                    std::cmp::Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error reading guess: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error reading guess: {}", e);
                continue;
            }
        };
    }
}
