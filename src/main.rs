use shell_words;

use std::io::{self};

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None, // EOF (Ctrl+D)
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

fn main() {
    let input = read_line().unwrap();
    match shell_words::split(&input) {
        Ok(tokens) => {
            for (i, token) in tokens.iter().enumerate() {
                println!("arg[{}]: '{}'", i, token);
            }
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
