mod commands;
mod utils;

use std::path::Path;

use commands::cat::*;
use commands::cd::cd;
use commands::echo::*;
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
            if value[0] == "exist" {
                exit();
            } else if value[0] == "echo" {
                echo(&value[1..]);
            } else if value[0] == "cat" {
                cat(&value[1..])
            } else if value[0] == "cd" {
                let path = Path::new(&value[1]);
                cd(&path) 
            }
        }
        Err(_) => println!("Incorrect input"),
    }
    // for token in tokens {
    //     println!("{}", token);
    // }
}
