use std::env;

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
    print_style();

    let home_dir = home_dir().unwrap_or("$".to_string());

    loop {
        let curret_dir = corrent_dir().unwrap_or("".to_string());
        print_currant_dir(&home_dir, curret_dir);
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
                match_command(&value, &home_dir);
            }
            Err(e) => println!("{e}"),
        }
    }
}

pub fn match_command(commands: &[String], home_dir: &str) {
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
                if commands[1] == "-" {
                    let old_dir = env::var("OLDPWD").unwrap_or("".to_string());
                    old_dir
                } else if commands[1] == "--" {
                    home_dir.to_string()
                } else {
                    commands[1].clone()
                }
            } else {
                home_dir.to_string()
            };
            cd(&path)
        }
        "pwd" => {
            if commands.len() > 1 {
                println!("pwd: too many arguments");
                return;
            }
            pwd()
        }
        "cp" => cp(&commands[1..]),
        "clear" => clear_terminal(),
        "mkdir" => mkdir(&commands[1..]),
        "rm" => rm(&commands[1..]),
        "mv" => mv(&commands[1..]),
        "ls" => ls(&commands[1..]),
        _ => print_error(&format!("Command '{comed}' not found")),
    }
}
