mod commands;
mod utils;
use commands::exit::*;
use commands::echo::*;
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
            if value[0] == "exist" {
                exit();
            } else if value[0] == "echo" {
                echo(&value[1..]);
            }
            // println!("{:?}" , value)
        }
        Err(_) => println!("Incorrect input"),
    }
    // for token in tokens {
    //     println!("{}", token);
    // }
}
