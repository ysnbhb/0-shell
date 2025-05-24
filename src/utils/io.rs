use std::io::stdin;

use crate::commands::exit::*;

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(0) => {
            exit();
            None
        }
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}
