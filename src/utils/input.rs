use std::io::{self, Write};

pub fn prompt_user(prompt: &str) ->String {
    print!("{}", prompt);
    io::stdout().flush().expect("Error clearing output buffer");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading entry");
    input.trim().to_string()
}