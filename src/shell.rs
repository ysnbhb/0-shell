use crate::commands::cat::*;
use crate::commands::cd::*;
use crate::commands::clear::clear_terminal;
use crate::commands::cp::cp;
use crate::commands::echo::*;
use crate::commands::ls::commend::ls;
use crate::commands::mkdir::mkdir;
use crate::commands::mv::mv;
use crate::commands::pwd::*;
use crate::commands::rm::rm;
use crate::utils::error::print_error;
use crate::utils::fs::*;
use crate::utils::io::*;
use crate::utils::parser::*;

pub fn shell() {
    clear_terminal();
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
        print_currant_dir(home_dir.clone(), curret_dir);
        let input = match read_line() {
            Some(mut text) => {
                text.pop();
                text
            }
            None => {
                println!();
                break;
            }
        };
        let tokens = parst_input(input, home_dir.clone());

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
            Err(e) => println!("{e}"),
        }
    }
}

pub fn match_command(commands: &[String], home_dir: String) {
    let comed = commands[0].clone();
    match comed.as_str() {
        "echo" => echo(&commands[1..]),
        "cat" => cat(&commands[1..]),
        "cd" => {
            if commands.len() > 2 {
                println!("cd: too many arguments");
                return;
            }
            let path = if commands.len() == 2 {
                &commands[1]
            } else {
                &home_dir
            };
            cd(path)
        }
        "pwd" => pwd(),
        "cp" => cp(&commands[1..]),
        "clear" => clear_terminal(),
        "mkdir" => mkdir(&commands[1..]),
        "rm" => rm(&commands[1..]),
        "mv" => mv(&commands[1..]),
        "ls" => ls(&commands[1..]),
        _ => print_error(&format!("Command '{comed}' not found")),
    }
}
