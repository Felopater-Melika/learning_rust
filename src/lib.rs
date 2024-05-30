use std::io;

pub fn read_input(prompt: &str) -> Result<String, io::Error> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
