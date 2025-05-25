mod commands;
mod utils;

use commands::cat::*;
use commands::cd::*;
use commands::echo::*;
use commands::exit::*;
use commands::pwd::*;
use utils::fs::home_dir;
use utils::io::*;
use utils::parser::*;

fn main() {
    let input = read_line().unwrap();
    let tokens = parst_input(input);
    let res = home_dir();

    let mut home_dir = String::new();

    match res {
        Some(hom) => home_dir = hom,
        None => {
            println!("filed to git home dir");
            return;
        }
    }
    match tokens {
        Ok(value) => {
            if value.len() < 1 {
                return;
            }
            if value[0] == "exit" {
                exit();
            } else if value[0] == "echo" {
                echo(&value[1..]);
            } else if value[0] == "cat" {
                cat(&value[1..])
            } else if value[0] == "cd" {
                let path = if value.len() == 2 {
                    &value[1]
                } else {
                    &home_dir
                };
                cd(&path);
            } else if value[0] == "pwd" {
                pwd();
            }
        }
        Err(_) => println!("Incorrect input"),
    }
    // for token in tokens {
    //     println!("{}", token);
    // }
}
