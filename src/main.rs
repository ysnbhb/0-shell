mod commands;
mod utils;
use commands::exit::*;
use utils::io::*;
use utils::parser::*;

fn main() {
    let input = read_line().unwrap();
    let tokens = parst_input(input);

    match tokens {
        Ok(value) => {
            if value.len() < 1 {
                return;
            }
            if value[0] == "exist" {}
            // println!("{:?}" , value)
            exit();
        }
        Err(_) => println!("Incorrect input"),
    }
    // for token in tokens {
    //     println!("{}", token);
    // }
}
