use std::io;
use std::io::Write;

use crate::commands::cat::*;
use crate::commands::cd::*;
use crate::commands::cp::cp;
use crate::commands::echo::*;
use crate::commands::pwd::*;
use crate::utils::fs::*;
use crate::utils::io::*;
use crate::utils::parser::*;

pub fn shell() {
    let home_dir_path = match home_dir() {
        Some(path) => path,
        None => {
            println!("failed to get home dir");
            return;
        }
    };
    let home_dir = home_dir_path.to_string();
    loop {
        let curret_dir = match corrent_dir() {
            Some(dir) => dir,
            None => {
                println!("failed to get current dir");
                break;
            }
        };
        print!("{}:$ ", curret_dir.replace(&home_dir, "~"));
        io::stdout().flush().unwrap();
        let input = match read_line() {
            Some(text) => text,
            None => {
                break;
            }
        };
        let tokens = parst_input(input.trim().to_string());

        match tokens {
            Ok(value) => {
                if value.is_empty() {
                    continue;
                }
                if value[0] == "exit" {
                    break;
                }
                match_command(&value, home_dir.clone());
            }
            Err(_) => println!("Incorrect input"),
        }
    }
}

fn match_command(commands: &[String], home_dir: String) {
    let comed = commands[0].clone();
    match comed.as_str() {
        "echo" => echo(&commands[1..]),
        "cat" => cat(&commands[1..]),
        "cd" => {
            let path = if commands.len() == 2 {
                &commands[1].replace("~", &home_dir)
            } else {
                &home_dir
            };
            cd(path)
        }
        "pwd" => pwd(),
        "cp" => cp(&commands[1..]),
        _ => println!("Command '{comed}' not found"),
    }
}
