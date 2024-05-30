use std::fs;
use learning_rust::read_input;

pub fn mini_grep() {
    let file_name = match read_input("Enter the name of the file you want to search:") {
        Ok(file_name) => file_name,
        Err(e) => {
            eprintln!("Error reading file name: {}", e);
            return;
        }
    };

    let word = match read_input("Enter the word you want to search:") {
        Ok(word) => word.trim().to_lowercase(),
        Err(e) => {
            eprintln!("Error reading word: {}", e);
            return;
        }
    };

    let file = fs::read_to_string(file_name);

    match file {
        Ok(file) => {
            for (i, line) in file.lines().enumerate() {
                if line.contains(&word) {
                    println!("{}: {}", i, line);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
